// Generated macro for impl_147 (impl)
macro_rules! Depcrate_oneshotimpl_147 {
() => {
// Module: crate::oneshot
// Provides: {"impl_147"}
// Dependencies: {}
impl < T > fmt :: Debug for Sender < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Sender") . field ("complete" , & self . inner . complete) . finish () } }
};
}
