use std::fs;

fn main() {
    println!("🧪 Testing individual problematic files");
    
    // Test Case 1: Macro expansion error
    test_macro_expansion_error();
    
    // Test Case 2: Missing Repr types
    test_missing_repr_types();
    
    // Test Case 3: Unresolved imports
    test_unresolved_imports();
    
    // Test Case 4: Generic type mismatch
    test_generic_type_mismatch();
}

fn test_macro_expansion_error() {
    println!("\n🔍 Test Case 1: Macro Expansion Error");
    println!("File: wrapped-zerocopy/src/decls/simd.rs:14:593");
    
    let test_code = r#"
// Test minimal macro expansion issue
macro_rules! test_macro {
    () => { include!("../../output2/wrapped-zerocopy/src/decls/simd.rs"); };
}

fn test() {
    test_macro!();
}
"#;
    
    write_and_test("test_macro_expansion.rs", test_code);
}

fn test_missing_repr_types() {
    println!("\n🔍 Test Case 2: Missing Repr Types");
    println!("Error: unresolved import `PrimitiveRepr`");
    
    let test_code = r#"
// Test missing Repr types
use PrimitiveRepr;
use AlignRepr;
use CompoundRepr;
use RawRepr;

fn test() {
    println!("Testing Repr types");
}
"#;
    
    write_and_test("test_missing_repr.rs", test_code);
}

fn test_unresolved_imports() {
    println!("\n🔍 Test Case 3: Unresolved Imports");
    println!("Error: unresolved import `super::ClockSequence`");
    
    let test_code = r#"
// Test unresolved super imports
use super::ClockSequence;

fn test() {
    println!("Testing super imports");
}
"#;
    
    write_and_test("test_unresolved_imports.rs", test_code);
}

fn test_generic_type_mismatch() {
    println!("\n🔍 Test Case 4: Generic Type Mismatch");
    println!("Error: enum takes 2 generic arguments but 1 generic argument was supplied");
    
    let test_code = r#"
// Test generic type mismatch
use std::result::Result;

fn test() {
    let _result: Result<i32> = Ok(42); // Missing Error type
}
"#;
    
    write_and_test("test_generic_mismatch.rs", test_code);
}

fn write_and_test(filename: &str, code: &str) {
    // Write test file
    if let Err(e) = fs::write(filename, code) {
        println!("❌ Failed to write {}: {}", filename, e);
        return;
    }
    
    // Try to compile
    let output = std::process::Command::new("rustc")
        .args(&["--crate-type", "lib", filename])
        .output();
    
    match output {
        Ok(result) if result.status.success() => {
            println!("✅ {} compiled successfully", filename);
        }
        Ok(result) => {
            println!("❌ {} failed to compile:", filename);
            let stderr = String::from_utf8_lossy(&result.stderr);
            // Show first few lines of error
            for line in stderr.lines().take(3) {
                println!("   {}", line);
            }
        }
        Err(e) => {
            println!("❌ System error testing {}: {}", filename, e);
        }
    }
    
    // Clean up
    let _ = fs::remove_file(filename);
    let _ = fs::remove_file(filename.replace(".rs", ".rlib"));
}
