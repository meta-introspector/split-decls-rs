// Generated macro for impl_79 (impl)
macro_rules! Depcrate_errorimpl_79 {
() => {
// Module: crate::error
// Provides: {"impl_79"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < Error > for std :: io :: Error { fn from (from : Error) -> Self { Self :: from_raw_os_error (from . code () . 0) } }
};
}
