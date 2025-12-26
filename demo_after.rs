// Auto-generated process audit wrapper
// All process executions are now audited

macro_rules! audit_execute {
    ($cmd:expr) => {{
        let start_time = std::time::Instant::now();
        println!("🔍 AUDIT: Executing command at {:?}", start_time);
        println!("📋 Command: {}", stringify!($cmd));
        
        let result = $cmd;
        
        let duration = start_time.elapsed();
        match &result {
            Ok(output) => {
                println!("✅ SUCCESS: Command completed in {:?}", duration);
                if let Some(status) = output.status.code() {
                    println!("📤 Exit code: {}", status);
                }
                if !output.stdout.is_empty() {
                    println!("📝 Stdout: {}", String::from_utf8_lossy(&output.stdout));
                }
                if !output.stderr.is_empty() {
                    println!("⚠️ Stderr: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => {
                println!("❌ ERROR: Command failed in {:?}: {}", duration, e);
            }
        }
        
        result
    }};
}

// Original code (before macro patch)
use std::process::Command;

fn main() {
    let output = audit_execute!(Command::new("ls").arg("-la").output()).unwrap();
    println!("Files: {}", String::from_utf8_lossy(&output.stdout));
    
    audit_execute!(Command::new("echo").arg("hello").status().unwrap();
}
