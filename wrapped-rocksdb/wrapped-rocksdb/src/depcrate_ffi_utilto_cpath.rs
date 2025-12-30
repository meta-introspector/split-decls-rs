// Generated macro for to_cpath (function)
macro_rules! Depcrate_ffi_utilto_cpath {
() => {
// Module: crate::ffi_util
// Provides: {"to_cpath"}
// Dependencies: {}
pub (crate) fn to_cpath < P : AsRef < Path > > (path : P) -> Result < CString , Error > { match CString :: new (path . as_ref () . to_string_lossy () . as_bytes ()) { Ok (c) => Ok (c) , Err (e) => Err (Error :: new (format ! ("Failed to convert path to CString: {e}"))) , } }
};
}
