// Generated macro for impl_964 (impl)
macro_rules! Depcrate_llvm_ffiimpl_964 {
() => {
// Module: crate::llvm::ffi
// Provides: {"impl_964"}
// Dependencies: {}
impl Debug for Bool { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . value { 0 => f . write_str ("FALSE") , 1 => f . write_str ("TRUE") , v => write ! (f , "TRUE ({v})") , } } }
};
}
