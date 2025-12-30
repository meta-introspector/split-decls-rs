// Generated macro for impl_724 (impl)
macro_rules! Depcrate_frameimpl_724 {
() => {
// Module: crate::frame
// Provides: {"impl_724"}
// Dependencies: {}
impl From < TransportError > for ConnectionClose { fn from (x : TransportError) -> Self { Self { error_code : x . code , frame_type : x . frame , reason : x . reason . into () , } } }
};
}
