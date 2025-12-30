// Generated macro for impl_57 (impl)
macro_rules! Depcrate_errorimpl_57 {
() => {
// Module: crate::error
// Provides: {"impl_57"}
// Dependencies: {}
# [cfg (not (feature = "custom-error-conversion"))] impl < T : Display + Send + Sync + 'static > From < T > for Error { fn from (e : T) -> Self { Self { message : e . to_string () , source : Some (Arc :: new (e)) , extensions : None , } } }
};
}
