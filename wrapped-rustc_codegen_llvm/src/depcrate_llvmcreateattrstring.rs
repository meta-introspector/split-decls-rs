// Generated macro for CreateAttrString (function)
macro_rules! Depcrate_llvmCreateAttrString {
() => {
// Module: crate::llvm
// Provides: {"CreateAttrString"}
// Dependencies: {}
pub (crate) fn CreateAttrString < 'll > (llcx : & 'll Context , attr : & str) -> & 'll Attribute { unsafe { LLVMCreateStringAttribute (llcx , attr . as_c_char_ptr () , attr . len () . try_into () . unwrap () , std :: ptr :: null () , 0 ,) } }
};
}
