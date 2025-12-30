// Generated macro for write_function_mappings_to_buffer (function)
macro_rules! Depcrate_coverageinfo_llvm_covwrite_function_mappings_to_buffer {
() => {
// Module: crate::coverageinfo::llvm_cov
// Provides: {"write_function_mappings_to_buffer"}
// Dependencies: {}
pub (crate) fn write_function_mappings_to_buffer (virtual_file_mapping : & [u32] , expressions : & [ffi :: CounterExpression] , regions : & ffi :: Regions ,) -> Vec < u8 > { let ffi :: Regions { code_regions , expansion_regions , branch_regions } = regions ; llvm :: build_byte_buffer (| buffer | unsafe { llvm :: LLVMRustCoverageWriteFunctionMappingsToBuffer (virtual_file_mapping . as_ptr () , virtual_file_mapping . len () , expressions . as_ptr () , expressions . len () , code_regions . as_ptr () , code_regions . len () , expansion_regions . as_ptr () , expansion_regions . len () , branch_regions . as_ptr () , branch_regions . len () , buffer ,) }) }
};
}
