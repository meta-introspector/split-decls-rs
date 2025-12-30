// Generated macro for create_member_type (function)
macro_rules! Depcrate_debuginfo_metadatacreate_member_type {
() => {
// Module: crate::debuginfo::metadata
// Provides: {"create_member_type"}
// Dependencies: {}
fn create_member_type < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , owner : & 'll DIScope , name : & str , file_metadata : & 'll DIType , line_number : u32 , layout : TyAndLayout < 'tcx > , offset : Size , flags : DIFlags , type_di_node : & 'll DIType ,) -> & 'll DIType { unsafe { llvm :: LLVMRustDIBuilderCreateMemberType (DIB (cx) , owner , name . as_c_char_ptr () , name . len () , file_metadata , line_number , layout . size . bits () , layout . align . abi . bits () as u32 , offset . bits () , flags , type_di_node ,) } }
};
}
