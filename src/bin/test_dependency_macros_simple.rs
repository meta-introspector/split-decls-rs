// Include the generated dependency macros
include!("dependency_macros.rs");

fn main() {
    println!("🧪 Testing dependency import macros");
    
    // Test the actual execution macro (this will show the real count)
    execute_all_deps!();
    
    println!("✅ Macro system working correctly");
    println!("📊 All dependencies can now be imported via macros");
}
