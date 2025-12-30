// Include the generated dependency macros
include!("dependency_macros.rs");

fn main() {
    println!("🔧 Executing rustc dependencies via generated macros");
    
    // Actually call the generated macro that executes all dependencies
    execute_all_deps!();
    
    println!("✅ All dependencies executed via real macro calls");
}
