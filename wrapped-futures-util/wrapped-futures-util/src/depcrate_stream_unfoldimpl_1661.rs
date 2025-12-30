// Generated macro for impl_1661 (impl)
macro_rules! Depcrate_stream_unfoldimpl_1661 {
() => {
// Module: crate::stream::unfold
// Provides: {"impl_1661"}
// Dependencies: {}
impl < T , F , Fut > fmt :: Debug for Unfold < T , F , Fut > where T : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Unfold") . field ("state" , & self . state) . finish () } }
};
}
