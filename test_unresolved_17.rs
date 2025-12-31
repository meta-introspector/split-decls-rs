// Test case for unresolved dependency: Some
// Similar matches found in symbol database:
// - rustc_codegen_cranelift::mini_core_hello_world::SomeTrait
// - rustc_resolve::errors::BindingShadowsSomethingUnacceptable
// - rustc_resolve::errors::BindingShadowsSomethingUnacceptableSuggestion

fn test_Some() {
    // Call: Some;
    println!("Testing dependency resolution");
}
