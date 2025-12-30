// Generated macro for impl_32 (impl)
macro_rules! Depcrate_errorimpl_32 {
() => {
// Module: crate::error
// Provides: {"impl_32"}
// Dependencies: {}
impl Error { # [cfg (unix)] pub (crate) fn unknown (message : impl Into < String > , err : impl Into < String >) -> Error { Self :: Other { message : message . into () , err : err . into () , } } }
};
}
