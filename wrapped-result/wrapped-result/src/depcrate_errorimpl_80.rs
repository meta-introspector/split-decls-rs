// Generated macro for impl_80 (impl)
macro_rules! Depcrate_errorimpl_80 {
() => {
// Module: crate::error
// Provides: {"impl_80"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < std :: io :: Error > for Error { fn from (from : std :: io :: Error) -> Self { match from . raw_os_error () { Some (status) => WIN32_ERROR (status as u32) . into () , None => HRESULT (E_UNEXPECTED) . into () , } } }
};
}
