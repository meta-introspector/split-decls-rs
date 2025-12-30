// Generated macro for impl_223 (impl)
macro_rules! Depcrate_multiimpl_223 {
() => {
// Module: crate::multi
// Provides: {"impl_223"}
// Dependencies: {}
impl fmt :: Debug for SocketEvents { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Events") . field ("input" , & self . input ()) . field ("output" , & self . output ()) . field ("remove" , & self . remove ()) . finish () } }
};
}
