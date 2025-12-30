// Generated macro for impl_11 (impl)
macro_rules! Depcrate_backtraceimpl_11 {
() => {
// Module: crate::backtrace
// Provides: {"impl_11"}
// Dependencies: {}
impl fmt :: Debug for Frame { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Frame") . field ("ip" , & self . ip ()) . field ("symbol_address" , & self . symbol_address ()) . finish () } }
};
}
