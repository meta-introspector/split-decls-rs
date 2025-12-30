// Generated macro for impl_142 (impl)
macro_rules! Depcrate_future_future_remote_handleimpl_142 {
() => {
// Module: crate::future::future::remote_handle
// Provides: {"impl_142"}
// Dependencies: {}
impl < Fut : Future + fmt :: Debug > fmt :: Debug for Remote < Fut > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Remote") . field (& self . future) . finish () } }
};
}
