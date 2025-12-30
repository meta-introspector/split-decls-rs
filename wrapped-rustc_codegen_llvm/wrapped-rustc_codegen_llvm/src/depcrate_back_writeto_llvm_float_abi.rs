// Generated macro for to_llvm_float_abi (function)
macro_rules! Depcrate_back_writeto_llvm_float_abi {
() => {
// Module: crate::back::write
// Provides: {"to_llvm_float_abi"}
// Dependencies: {}
fn to_llvm_float_abi (float_abi : Option < FloatAbi >) -> llvm :: FloatAbi { match float_abi { None => llvm :: FloatAbi :: Default , Some (FloatAbi :: Soft) => llvm :: FloatAbi :: Soft , Some (FloatAbi :: Hard) => llvm :: FloatAbi :: Hard , } }
};
}
