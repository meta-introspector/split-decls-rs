// Generated macro for create_subroutine_type (function)
macro_rules! Depcrate_debuginfo_metadatacreate_subroutine_type {
() => {
// Module: crate::debuginfo::metadata
// Provides: {"create_subroutine_type"}
// Dependencies: {}
pub (super) fn create_subroutine_type < 'll > (cx : & CodegenCx < 'll , '_ > , signature : & 'll DICompositeType ,) -> & 'll DICompositeType { unsafe { llvm :: LLVMRustDIBuilderCreateSubroutineType (DIB (cx) , signature) } }
};
}
