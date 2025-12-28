macro_rules! validate_rust_code {
    () => {
        pub async fn validate_rust_code (file_path : & PathBuf) -> anyhow :: Result < () > { let output = Command :: new ("rustc") . arg ("--emit=metadata") . arg ("--crate-type=lib") . arg (file_path) . output () . await . context ("Failed to execute rustc") ? ; if ! output . status . success () { let stderr = String :: from_utf8_lossy (& output . stderr) ; anyhow :: bail ! ("Rustc check failed for file {:?}:\n{}" , file_path , stderr) ; } Ok (()) }
    };
}

validate_rust_code!()