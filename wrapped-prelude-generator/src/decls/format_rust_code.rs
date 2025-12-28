macro_rules! format_rust_code {
    () => {
        pub async fn format_rust_code (file_path : & PathBuf) -> anyhow :: Result < () > { let output = Command :: new ("rustfmt") . arg (file_path) . arg ("--edition=2021") . arg ("--emit=files") . output () . await . context ("Failed to execute rustfmt") ? ; if ! output . status . success () { let stderr = String :: from_utf8_lossy (& output . stderr) ; anyhow :: bail ! ("Rustfmt failed for file {:?}:\n{}" , file_path , stderr) ; } Ok (()) }
    };
}

format_rust_code!()