fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir: std::path::PathBuf = std::env::current_dir()?;
    let path = current_dir
        .iter()
        .skip(1)
        .take(current_dir.iter().count() - 2)
        .fold(String::new(), |path, part| {
            let mut buf = [0; 4];
            path + "/"
                + part
                    .to_string_lossy()
                    .chars()
                    .nth(0)
                    .unwrap()
                    .encode_utf8(&mut buf)
        });
    println!(
        "{}",
        path + "/"
            + &current_dir
                .iter()
                .last()
                .ok_or("No last item")?
                .to_string_lossy()
    );
    Ok(())
}
