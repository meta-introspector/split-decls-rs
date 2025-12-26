// Test program to demonstrate full bootstrap audit
use std::process::Command;
use std::fs;

// Include audit macros
include!("bootstrap_audit_macros.rs");

fn main() {
    println!("🧪 Testing Full Bootstrap Audit System");
    println!("======================================\n");
    
    // Test process execution audit
    println!("1. Testing process execution audit:");
    let _output = audit_execute!(Command::new("echo").arg("Bootstrap test").output()).unwrap();
    
    println!("\n2. Testing file write audit:");
    audit_fs_write!("test_bootstrap_file.txt", b"Bootstrap audit test content").unwrap();
    
    println!("\n3. Testing directory creation audit:");
    audit_fs_create_dir_all!("test_bootstrap_dir").unwrap();
    
    println!("\n✅ All audit tests completed!");
}
