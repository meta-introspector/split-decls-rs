// Test case for unresolved dependency: process::exit
// Similar matches found in symbol database:
// - rustc_pattern_analysis::complexity::test_big_enum
// - rustc_pattern_analysis::complexity::use_common___*
// - rustc_codegen_gcc::exit_code::main

// Expected: use process;
fn test_exit() {
    // Call: process::exit;
    println!("Testing dependency resolution");
}
