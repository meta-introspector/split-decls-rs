// Generated macro for impl_98 (impl)
macro_rules! Depcrate_streamimpl_98 {
() => {
// Module: crate::stream
// Provides: {"impl_98"}
// Dependencies: {}
impl < T , F , Fut > fmt :: Debug for Unfold < T , F , Fut > where T : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Unfold") . field ("state" , & self . state) . field ("fut" , & self . fut) . finish () } }
};
}
