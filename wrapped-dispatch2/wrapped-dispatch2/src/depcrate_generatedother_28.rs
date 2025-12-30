// Generated macro for other_28 (other)
macro_rules! Depcrate_generatedother_28 {
() => {
// Module: crate::generated
// Provides: {"other_28"}
// Dependencies: {}
extern "C" { # [doc = " Associates an application defined context with the object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `object`: The result of passing NULL in this parameter is undefined."] # [doc = ""] # [doc = ""] # [doc = " Parameter `context`: The new client defined context for the object. This may be NULL."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `object` must be a valid pointer."] # [doc = " - `context` must be a valid pointer or null."] pub fn dispatch_set_context (object : NonNull < dispatch_object_s > , context : * mut c_void) ; }
};
}
