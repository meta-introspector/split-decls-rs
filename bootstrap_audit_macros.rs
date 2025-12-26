// Comprehensive syscall audit macros for bootstrap
use std::time::{SystemTime, Instant};

macro_rules! audit_execute {
    ($cmd:expr) => {{
        let start = Instant::now();
        let timestamp = SystemTime::now();
        println!("⚠️  PROCESS AUDIT: {:?}", timestamp);
        println!("📋 Command: {}", stringify!($cmd));
        println!("📁 PWD: {:?}", std::env::current_dir().unwrap_or_default());
        
        let result = $cmd;
        let duration = start.elapsed();
        
        match &result {
            Ok(output) => {
                println!("✅ Process completed in {:?}", duration);
                if let Some(code) = output.status.code() {
                    println!("📤 Exit: {}", code);
                }
            }
            Err(e) => println!("❌ Process failed: {}", e),
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        result
    }};
}

macro_rules! audit_fs_write {
    ($path:expr, $contents:expr) => {{
        let timestamp = SystemTime::now();
        println!("⚠️  FILE WRITE AUDIT: {:?}", timestamp);
        println!("📝 Writing to: {:?}", $path);
        println!("📊 Size: {} bytes", $contents.len());
        
        let result = std::fs::write($path, $contents);
        
        match &result {
            Ok(_) => println!("✅ File written successfully"),
            Err(e) => println!("❌ File write failed: {}", e),
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        result
    }};
}

macro_rules! audit_fs_create_dir_all {
    ($path:expr) => {{
        let timestamp = SystemTime::now();
        println!("⚠️  DIR CREATE AUDIT: {:?}", timestamp);
        println!("📁 Creating: {:?}", $path);
        
        let result = std::fs::create_dir_all($path);
        
        match &result {
            Ok(_) => println!("✅ Directory created successfully"),
            Err(e) => println!("❌ Directory creation failed: {}", e),
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        result
    }};
}
