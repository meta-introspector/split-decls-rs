// Generated macro for dispatch_get_context (function)
macro_rules! Depcrate_generateddispatch_get_context {
() => {
// Module: crate::generated
// Provides: {"dispatch_get_context"}
// Dependencies: {}
# [doc = " Returns the application defined context of the object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `object`: The result of passing NULL in this parameter is undefined."] # [doc = ""] # [doc = ""] # [doc = " Returns: The context of the object; may be NULL."] # [must_use] # [inline] pub extern "C" fn dispatch_get_context (object : NonNull < dispatch_object_s >) -> * mut c_void { extern "C" { fn dispatch_get_context (object : NonNull < dispatch_object_s >) -> * mut c_void ; } unsafe { dispatch_get_context (object) } }
};
}
