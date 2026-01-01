// Test file to demonstrate AST patch proof generation

#[warn(unused_variables)] // AST_test_file_TRAIT_9999
pub trait TestTrait {
    // This will be patched to demonstrate proof generation
    pub(in crate::invalid) fn broken_visibility() -> Self;
}

#[warn(unused_variables)] // AST_test_file_FN_8888  
pub fn test_function() {
    unsafe unsafe {
        // Duplicate unsafe keyword to test patching
        println!("Testing patch system");
    }
}
