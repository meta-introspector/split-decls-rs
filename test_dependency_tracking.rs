// Test the dependency tracking macro system
include!("src/macro_wrappers.rs");

// Test function with dependency tracking
macro_rules! test_function_introspect {
    () => {
        println!("📊 INTROSPECT: Function test_function in module {}", module_path!());
        println!("📋 DEPS: std::println");
        println!("📦 CRATES: std");
    };
}

mkfn!{
    test_function_introspect!();
    fn test_function() {
        println!("Hello from tracked function!");
    }
}

// Test module with dependency tracking
macro_rules! test_module_getname { () => { "test_module" }; }
macro_rules! test_module_getsrc { () => { "test_dependency_tracking.rs" }; }
macro_rules! test_module_getpath { () => { "./test_dependency_tracking.rs" }; }
macro_rules! test_module_include { () => { "test_dependency_tracking.rs" }; }
macro_rules! test_module_get_deps { () => { vec!["std::println"] }; }
macro_rules! test_module_get_crates { () => { vec!["std"] }; }

mkmod!{
    pub mod test_module {
        pub fn inner_function() {
            println!("Inner function called");
        }
    }
}

fn main() {
    println!("🚀 Testing dependency tracking system");
    
    // Test function call interception
    test_function();
    
    // Test introspection macros
    println!("📊 Module name: {}", test_module_getname!());
    println!("📊 Module source: {}", test_module_getsrc!());
    println!("📊 Module path: {}", test_module_getpath!());
    println!("📊 Module deps: {:?}", test_module_get_deps!());
    println!("📊 Module crates: {:?}", test_module_get_crates!());
    
    // Test module access
    test_module::inner_function();
    
    println!("✅ Dependency tracking test complete");
}
