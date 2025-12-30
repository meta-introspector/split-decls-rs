// Generated macro for CaptureConnection (struct)
macro_rules! Depcrate_client_legacy_connect_captureCaptureConnection {
() => {
// Module: crate::client::legacy::connect::capture
// Provides: {"CaptureConnection"}
// Dependencies: {}
# [doc = " [`CaptureConnection`] allows callers to capture [`Connected`] information"] # [doc = ""] # [doc = " To capture a connection for a request, use [`capture_connection`]."] # [derive (Debug , Clone)] pub struct CaptureConnection { rx : watch :: Receiver < Option < Connected > > , }
};
}
