// Generated macro for impl_131 (impl)
macro_rules! Depcrate_taskimpl_131 {
() => {
// Module: crate::task
// Provides: {"impl_131"}
// Dependencies: {}
impl < T , M : fmt :: Debug > fmt :: Debug for Task < T , M > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Task") . field ("header" , self . header_with_metadata ()) . finish () } }
};
}
