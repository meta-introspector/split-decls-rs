// Generated macro for impl_301 (impl)
macro_rules! Depcrate_client_legacy_connect_captureimpl_301 {
() => {
// Module: crate::client::legacy::connect::capture
// Provides: {"impl_301"}
// Dependencies: {}
impl CaptureConnection { # [doc = " Internal API to create the tx and rx half of [`CaptureConnection`]"] pub (crate) fn new () -> (CaptureConnectionExtension , Self) { let (tx , rx) = watch :: channel (None) ; (CaptureConnectionExtension { tx : Arc :: new (tx) } , CaptureConnection { rx } ,) } # [doc = " Retrieve the connection metadata, if available"] pub fn connection_metadata (& self) -> impl Deref < Target = Option < Connected > > + '_ { self . rx . borrow () } # [doc = " Wait for the connection to be established"] # [doc = ""] # [doc = " If a connection was established, this will always return `Some(...)`. If the request never"] # [doc = " successfully connected (e.g. DNS resolution failure), this method will never return."] pub async fn wait_for_connection_metadata (& mut self ,) -> impl Deref < Target = Option < Connected > > + '_ { if self . rx . borrow () . is_some () { return self . rx . borrow () ; } let _ = self . rx . changed () . await ; self . rx . borrow () } }
};
}
