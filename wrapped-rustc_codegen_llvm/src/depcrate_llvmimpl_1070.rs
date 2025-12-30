// Generated macro for impl_1070 (impl)
macro_rules! Depcrate_llvmimpl_1070 {
() => {
// Module: crate::llvm
// Provides: {"impl_1070"}
// Dependencies: {}
impl Intrinsic { pub (crate) fn lookup (name : & [u8]) -> Option < Self > { let id = unsafe { LLVMLookupIntrinsicID (name . as_c_char_ptr () , name . len ()) } ; NonZero :: new (id) . map (| id | Self { id }) } pub (crate) fn get_declaration < 'll > (self , llmod : & 'll Module , type_params : & [& 'll Type] ,) -> & 'll Value { unsafe { LLVMGetIntrinsicDeclaration (llmod , self . id , type_params . as_ptr () , type_params . len ()) } } }
};
}
