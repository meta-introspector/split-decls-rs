use module_wrapper_macros::includemod;

// Create a simple test file to include
pub mod split_decls_config_mod {
    #[derive(Debug, Clone)]
    pub struct SplitDeclsConfig {
        pub test_field: String,
    }
}

// Test the includemod! macro
includemod!(test_mod, "test_include.rs");

fn main() {
    println!("Testing includemod! macro");
    
    // Test that the module was created
    let result = test_mod::test_function();
    println!("Result: {:?}", result);
}
