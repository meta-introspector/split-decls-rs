// Generated macro for SCErrorString (function)
macro_rules! Depcrate_generatedSCErrorString {
() => {
// Module: crate::generated
// Provides: {"SCErrorString"}
// Dependencies: {}
# [doc = " Returns a pointer to the message string"] # [doc = " associated with the specified status or error"] # [doc = " number."] # [doc = ""] # [doc = " Parameter `status`: The status or error number."] # [doc = ""] # [doc = " Returns: Returns a pointer to the error message string."] # [inline] pub extern "C-unwind" fn SCErrorString (status : c_int) -> NonNull < c_char > { extern "C-unwind" { fn SCErrorString (status : c_int) -> Option < NonNull < c_char > > ; } let ret = unsafe { SCErrorString (status) } ; ret . expect ("function was marked as returning non-null, but actually returned NULL") }
};
}
