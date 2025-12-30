// Generated macro for impl_501 (impl)
macro_rules! Depcrate_threadimpl_501 {
() => {
// Module: crate::thread
// Provides: {"impl_501"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for JoinHandle < T > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("JoinHandle") . finish () } }
};
}
