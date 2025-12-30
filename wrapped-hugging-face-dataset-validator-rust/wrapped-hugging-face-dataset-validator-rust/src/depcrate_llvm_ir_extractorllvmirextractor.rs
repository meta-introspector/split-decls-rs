// Generated macro for LLVMIRExtractor (struct)
macro_rules! Depcrate_llvm_ir_extractorLLVMIRExtractor {
() => {
// Module: crate::llvm_ir_extractor
// Provides: {"LLVMIRExtractor"}
// Dependencies: {}
# [doc = " Main extractor for LLVM IR analysis data"] # [doc = " "] # [doc = " This extractor analyzes the Rust → LLVM IR compilation process,"] # [doc = " capturing IR generation, optimization passes, and code generation"] # [doc = " to create rich datasets for machine learning applications."] pub struct LLVMIRExtractor { # [doc = " Version of the extractor tool"] extractor_version : String , # [doc = " Version of LLVM being used"] llvm_version : String , # [doc = " Version of Rust compiler"] rustc_version : String , # [doc = " Processing order counter"] processing_order : u32 , }
};
}
