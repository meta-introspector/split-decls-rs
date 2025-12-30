// Generated macro for impl_300 (impl)
macro_rules! Depcrate_client_legacy_connect_captureimpl_300 {
() => {
// Module: crate::client::legacy::connect::capture
// Provides: {"impl_300"}
// Dependencies: {}
impl CaptureConnectionExtension { pub (crate) fn set (& self , connected : & Connected) { self . tx . send_replace (Some (connected . clone ())) ; } }
};
}
