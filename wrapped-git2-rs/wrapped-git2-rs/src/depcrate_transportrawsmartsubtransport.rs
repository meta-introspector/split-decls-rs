// Generated macro for RawSmartSubtransport (struct)
macro_rules! Depcrate_transportRawSmartSubtransport {
() => {
// Module: crate::transport
// Provides: {"RawSmartSubtransport"}
// Dependencies: {}
# [doc = " Instance of a `git_smart_subtransport`, must use `#[repr(C)]` to ensure that"] # [doc = " the C fields come first."] # [repr (C)] struct RawSmartSubtransport { raw : raw :: git_smart_subtransport , stream : Option < * mut raw :: git_smart_subtransport_stream > , rpc : bool , obj : Box < dyn SmartSubtransport > , }
};
}
