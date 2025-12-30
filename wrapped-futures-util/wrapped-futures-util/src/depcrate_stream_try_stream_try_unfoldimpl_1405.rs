// Generated macro for impl_1405 (impl)
macro_rules! Depcrate_stream_try_stream_try_unfoldimpl_1405 {
() => {
// Module: crate::stream::try_stream::try_unfold
// Provides: {"impl_1405"}
// Dependencies: {}
impl < T , F , Fut > fmt :: Debug for TryUnfold < T , F , Fut > where T : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TryUnfold") . field ("state" , & self . state) . field ("fut" , & self . fut) . finish () } }
};
}
