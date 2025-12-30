// Generated macro for tests (module)
macro_rules! Depcrate_llvm_ir_extractortests {
() => {
// Module: crate::llvm_ir_extractor
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use tempfile :: TempDir ; use std :: fs ; # [test] fn test_llvm_ir_extractor_creation () { let extractor = LLVMIRExtractor :: new () ; assert ! (extractor . is_ok ()) ; } # [test] fn test_ir_generation_extraction () { let temp_dir = TempDir :: new () . unwrap () ; let source_file = temp_dir . path () . join ("test.rs") ; fs :: write (& source_file , r#"
fn main() {
    println!("Hello, world!");
}
"#) . unwrap () ; let mut extractor = LLVMIRExtractor :: new () . unwrap () ; let records = extractor . extract_ir_generation (& source_file , "O0") . unwrap () ; assert_eq ! (records . len () , 1) ; assert_eq ! (records [0] . optimization_level , "O0") ; assert_eq ! (records [0] . target_architecture , "x86_64") ; assert ! (records [0] . llvm_ir . contains ("define void @main")) ; } }
};
}
