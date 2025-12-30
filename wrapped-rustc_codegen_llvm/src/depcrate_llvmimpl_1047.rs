// Generated macro for impl_1047 (impl)
macro_rules! Depcrate_llvmimpl_1047 {
() => {
// Module: crate::llvm
// Provides: {"impl_1047"}
// Dependencies: {}
impl AttributePlace { pub (crate) fn as_uint (self) -> c_uint { match self { AttributePlace :: ReturnValue => 0 , AttributePlace :: Argument (i) => 1 + i , AttributePlace :: Function => ! 0 , } } }
};
}
