// Generated macro for impl_95 (impl)
macro_rules! Depcrate_runnableimpl_95 {
() => {
// Module: crate::runnable
// Provides: {"impl_95"}
// Dependencies: {}
impl < M : fmt :: Debug > fmt :: Debug for Runnable < M > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let ptr = self . ptr . as_ptr () ; let header = ptr as * const HeaderWithMetadata < M > ; f . debug_struct ("Runnable") . field ("header" , unsafe { & (* header) }) . finish () } }
};
}
