// Generated macro for CaptureConnectionExtension (struct)
macro_rules! Depcrate_client_legacy_connect_captureCaptureConnectionExtension {
() => {
// Module: crate::client::legacy::connect::capture
// Provides: {"CaptureConnectionExtension"}
// Dependencies: {}
# [doc = " TxSide for [`CaptureConnection`]"] # [doc = ""] # [doc = " This is inserted into `Extensions` to allow Hyper to back channel connection info"] # [derive (Clone)] pub (crate) struct CaptureConnectionExtension { tx : Arc < watch :: Sender < Option < Connected > > > , }
};
}
