// AST Patch for FN_0052 - Fix unterminated string in consts.rs
// Error: unterminated double quote string at line 52
// Pattern: String literal with escaped quote at end causing parse error

use std::fs;

pub fn apply_patch() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "src/processed_rustc_codegen_llvm_rustc_codegen_llvm_src_consts.rs";
    let content = fs::read_to_string(file_path)?;
    
    // Fix the unterminated string by completing the assert message
    let fixed_content = content.replace(
        r#"assert ! (! defined_in_current_codegen_unit , "consts::get_static() should always hit the cache for \"#,
        r#"assert ! (! defined_in_current_codegen_unit , "consts::get_static() should always hit the cache for statics defined in the current codegen unit")"#
    );
    
    fs::write(file_path, fixed_content)?;
    println!("✅ Fixed unterminated string in consts.rs-0052");
    Ok(())
}
