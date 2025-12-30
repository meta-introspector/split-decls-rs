// Generated macro for impl_157 (impl)
macro_rules! Depcrate_oneshotimpl_157 {
() => {
// Module: crate::oneshot
// Provides: {"impl_157"}
// Dependencies: {}
impl < T > fmt :: Debug for Receiver < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Receiver") . field ("complete" , & self . inner . complete) . finish () } }
};
}
