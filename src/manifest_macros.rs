// Manifest reporting macro for binaries
// Add this to each binary to auto-generate manifest entries

#[macro_export]
macro_rules! report_manifest {
    ($binary_name:expr) => {
        pub fn generate_manifest_entry() -> toml::Value {
            let mut manifest = toml::Table::new();
            
            // Auto-detect source file
            manifest.insert("source_file".to_string(), 
                toml::Value::String(format!("src/bin/{}.rs", $binary_name)));
            
            // Auto-detect output file  
            manifest.insert("output_file".to_string(),
                toml::Value::String(format!("output2/wrapped-split-decls-rs/src/decls/wrapped_split_decls_rs_decls_{}.rs", $binary_name)));
            
            toml::Value::Table(manifest)
        }
    };
}

#[macro_export]
macro_rules! track_file_read {
    ($path:expr) => {
        eprintln!("MANIFEST_READ: {}", $path);
        #[syscall="read"]
    std::fs::read_to_string($path)
    };
}

#[macro_export]
macro_rules! track_file_write {
    ($path:expr, $content:expr) => {
        eprintln!("MANIFEST_WRITE: {}", $path);
        #[syscall="write"]
    std::fs::write($path, $content)
    };
}

#[macro_export]
macro_rules! track_command {
    ($cmd:expr) => {
        eprintln!("MANIFEST_COMMAND: {:?}", $cmd);
        $cmd
    };
}

// Usage in binaries:
// report_manifest!("report_aliases");
// let content = track_file_read!("split-decls-rs.toml")?;
// track_file_write!("output.txt", "data")?;
// track_command!(#[syscall="exec"]
    Command::new("cargo").arg("build"))
