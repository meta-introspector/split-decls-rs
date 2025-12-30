// Generated macro for impl_59 (impl)
macro_rules! Depcrate_errorimpl_59 {
() => {
// Module: crate::error
// Provides: {"impl_59"}
// Dependencies: {}
# [cfg (feature = "custom-error-conversion")] impl From < String > for Error { fn from (e : String) -> Self { Self { message : e , source : None , extensions : None , } } }
};
}
