// Generated macro for impl_138 (impl)
macro_rules! Depcrate_future_future_remote_handleimpl_138 {
() => {
// Module: crate::future::future::remote_handle
// Provides: {"impl_138"}
// Dependencies: {}
impl < T > RemoteHandle < T > { # [doc = " Drops this handle *without* canceling the underlying future."] # [doc = ""] # [doc = " This method can be used if you want to drop the handle, but let the"] # [doc = " execution continue."] pub fn forget (self) { self . keep_running . store (true , Ordering :: SeqCst) ; } }
};
}
