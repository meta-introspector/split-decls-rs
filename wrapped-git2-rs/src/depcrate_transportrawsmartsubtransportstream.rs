// Generated macro for RawSmartSubtransportStream (struct)
macro_rules! Depcrate_transportRawSmartSubtransportStream {
() => {
// Module: crate::transport
// Provides: {"RawSmartSubtransportStream"}
// Dependencies: {}
# [doc = " Instance of a `git_smart_subtransport_stream`, must use `#[repr(C)]` to"] # [doc = " ensure that the C fields come first."] # [repr (C)] struct RawSmartSubtransportStream { raw : raw :: git_smart_subtransport_stream , obj : Box < dyn SmartSubtransportStream > , }
};
}
