// Generated macro for impl_284 (impl)
macro_rules! Depcrateimpl_284 {
() => {
// Module: crate
// Provides: {"impl_284"}
// Dependencies: {}
# [cfg (not (feature = "rustc-dep-of-std"))] impl StreamResult { # [inline] pub const fn error (error : MZError) -> StreamResult { StreamResult { bytes_consumed : 0 , bytes_written : 0 , status : Err (error) , } } }
};
}
