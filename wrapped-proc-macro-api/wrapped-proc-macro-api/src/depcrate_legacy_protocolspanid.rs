// Generated macro for SpanId (struct)
macro_rules! Depcrate_legacy_protocolSpanId {
() => {
// Module: crate::legacy_protocol
// Provides: {"SpanId"}
// Dependencies: {}
# [doc = " Legacy span type, only defined here as it is still used by the proc-macro server."] # [doc = " While rust-analyzer doesn't use this anymore at all, RustRover relies on the legacy type for"] # [doc = " proc-macro expansion."] # [derive (Clone , Copy , PartialEq , Eq , Hash)] pub struct SpanId (pub u32) ;
};
}
