// Generated macro for impl_1172 (impl)
macro_rules! Depcrate_type_impl_1172 {
() => {
// Module: crate::type_
// Provides: {"impl_1172"}
// Dependencies: {}
impl fmt :: Debug for Type { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (& llvm :: build_string (| s | unsafe { llvm :: LLVMRustWriteTypeToString (self , s) ; }) . expect ("non-UTF8 type description from LLVM") ,) } }
};
}
