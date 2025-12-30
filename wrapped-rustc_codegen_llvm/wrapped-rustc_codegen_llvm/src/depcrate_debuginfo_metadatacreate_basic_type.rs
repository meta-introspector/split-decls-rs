// Generated macro for create_basic_type (function)
macro_rules! Depcrate_debuginfo_metadatacreate_basic_type {
() => {
// Module: crate::debuginfo::metadata
// Provides: {"create_basic_type"}
// Dependencies: {}
fn create_basic_type < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , name : & str , size : Size , encoding : u32 ,) -> & 'll DIBasicType { unsafe { llvm :: LLVMRustDIBuilderCreateBasicType (DIB (cx) , name . as_c_char_ptr () , name . len () , size . bits () , encoding ,) } }
};
}
