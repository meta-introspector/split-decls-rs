use std::process::Command;

macro_rules! audit_execute {
    ($cmd:expr) => {{
        println!("⚠️  WARNING: Process execution detected!");
        println!("📋 Command: {}", stringify!($cmd));
        println!("🕐 Time: {:?}", std::time::SystemTime::now());
        $cmd
    }};
}

fn main() {
    println!("🧪 Testing macro patch warnings...\n");
    
    let output = audit_execute!(Command::new("echo").arg("Hello World").output()).unwrap();
    println!("Result: {}", String::from_utf8_lossy(&output.stdout));
    
    audit_execute!(Command::new("date").status()).unwrap();
}
