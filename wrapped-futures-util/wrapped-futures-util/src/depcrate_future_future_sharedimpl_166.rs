// Generated macro for impl_166 (impl)
macro_rules! Depcrate_future_future_sharedimpl_166 {
() => {
// Module: crate::future::future::shared
// Provides: {"impl_166"}
// Dependencies: {}
impl < Fut : Future > fmt :: Debug for Shared < Fut > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Shared") . field ("inner" , & self . inner) . field ("waker_key" , & self . waker_key) . finish () } }
};
}
