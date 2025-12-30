// Generated macro for impl_1235 (impl)
macro_rules! Depcrate_valueimpl_1235 {
() => {
// Module: crate::value
// Provides: {"impl_1235"}
// Dependencies: {}
impl fmt :: Debug for Value { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (& llvm :: build_string (| s | unsafe { llvm :: LLVMRustWriteValueToString (self , s) ; }) . expect ("non-UTF8 value description from LLVM") ,) } }
};
}
