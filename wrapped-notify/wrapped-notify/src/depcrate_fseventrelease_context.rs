// Generated macro for release_context (function)
macro_rules! Depcrate_fseventrelease_context {
() => {
// Module: crate::fsevent
// Provides: {"release_context"}
// Dependencies: {}
extern "C" fn release_context (info : * const libc :: c_void) { unsafe { drop (Box :: from_raw (info as * const StreamContextInfo as * mut StreamContextInfo ,)) ; } }
};
}
