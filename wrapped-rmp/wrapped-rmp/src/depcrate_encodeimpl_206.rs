// Generated macro for impl_206 (impl)
macro_rules! Depcrate_encodeimpl_206 {
() => {
// Module: crate::encode
// Provides: {"impl_206"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < ValueWriteError < Self > > for std :: io :: Error { # [cold] fn from (err : ValueWriteError < Self >) -> Self { match err { ValueWriteError :: InvalidMarkerWrite (err) | ValueWriteError :: InvalidDataWrite (err) => err , } } }
};
}
