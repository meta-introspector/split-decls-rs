// Generated macro for impl_58 (impl)
macro_rules! Depcrate_errorimpl_58 {
() => {
// Module: crate::error
// Provides: {"impl_58"}
// Dependencies: {}
# [cfg (feature = "custom-error-conversion")] impl From < & 'static str > for Error { fn from (e : & 'static str) -> Self { Self { message : e . to_string () , source : None , extensions : None , } } }
};
}
