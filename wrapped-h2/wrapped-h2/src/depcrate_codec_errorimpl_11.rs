// Generated macro for impl_11 (impl)
macro_rules! Depcrate_codec_errorimpl_11 {
() => {
// Module: crate::codec::error
// Provides: {"impl_11"}
// Dependencies: {}
impl From < io :: Error > for SendError { fn from (src : io :: Error) -> Self { Self :: Connection (src . into ()) } }
};
}
