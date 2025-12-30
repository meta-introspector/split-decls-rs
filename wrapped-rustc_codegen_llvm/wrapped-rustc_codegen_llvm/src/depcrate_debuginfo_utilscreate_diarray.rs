// Generated macro for create_DIArray (function)
macro_rules! Depcrate_debuginfo_utilscreate_DIArray {
() => {
// Module: crate::debuginfo::utils
// Provides: {"create_DIArray"}
// Dependencies: {}
# [allow (non_snake_case)] pub (crate) fn create_DIArray < 'll > (builder : & DIBuilder < 'll > , arr : & [Option < & 'll DIDescriptor >] ,) -> & 'll DIArray { unsafe { llvm :: LLVMRustDIBuilderGetOrCreateArray (builder , arr . as_ptr () , arr . len () as u32) } }
};
}
