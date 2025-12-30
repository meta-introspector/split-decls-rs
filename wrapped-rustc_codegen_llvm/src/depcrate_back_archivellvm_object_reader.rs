// Generated macro for LLVM_OBJECT_READER (static)
macro_rules! Depcrate_back_archiveLLVM_OBJECT_READER {
() => {
// Module: crate::back::archive
// Provides: {"LLVM_OBJECT_READER"}
// Dependencies: {}
static LLVM_OBJECT_READER : ObjectReader = ObjectReader { get_symbols : get_llvm_object_symbols , is_64_bit_object_file : llvm_is_64_bit_object_file , is_ec_object_file : llvm_is_ec_object_file , is_any_arm64_coff : llvm_is_any_arm64_coff , get_xcoff_member_alignment : DEFAULT_OBJECT_READER . get_xcoff_member_alignment , } ;
};
}
