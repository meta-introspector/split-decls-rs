macro_rules! ProbeResult {
    () => {
        pub struct ProbeResult { pub cert_file : Option < PathBuf > , pub cert_dir : Option < PathBuf > , }
    };
}

ProbeResult!()