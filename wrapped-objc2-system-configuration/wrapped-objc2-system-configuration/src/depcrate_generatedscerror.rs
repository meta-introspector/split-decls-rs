// Generated macro for SCError (function)
macro_rules! Depcrate_generatedSCError {
() => {
// Module: crate::generated
// Provides: {"SCError"}
// Dependencies: {}
# [doc = " Returns the most recent status or error code generated"] # [doc = " as the result of calling a System Configuration framework API."] # [doc = ""] # [doc = " Returns: Returns the last error encountered."] # [inline] pub extern "C-unwind" fn SCError () -> c_int { extern "C-unwind" { fn SCError () -> c_int ; } unsafe { SCError () } }
};
}
