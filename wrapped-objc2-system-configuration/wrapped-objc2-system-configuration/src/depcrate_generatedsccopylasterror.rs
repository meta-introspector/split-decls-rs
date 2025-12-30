// Generated macro for SCCopyLastError (function)
macro_rules! Depcrate_generatedSCCopyLastError {
() => {
// Module: crate::generated
// Provides: {"SCCopyLastError"}
// Dependencies: {}
# [doc = " Returns the most recent status or error code generated"] # [doc = " as the result of calling a System Configuration framework API."] # [doc = ""] # [doc = " Returns: Returns the last error encountered."] # [inline] pub extern "C-unwind" fn SCCopyLastError () -> CFRetained < CFError > { extern "C-unwind" { fn SCCopyLastError () -> Option < NonNull < CFError > > ; } let ret = unsafe { SCCopyLastError () } ; let ret = ret . expect ("function was marked as returning non-null, but actually returned NULL") ; unsafe { CFRetained :: from_raw (ret) } }
};
}
