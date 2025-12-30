// Generated macro for dispatch_function_t (type)
macro_rules! Depcratedispatch_function_t {
() => {
// Module: crate
// Provides: {"dispatch_function_t"}
// Dependencies: {}
# [doc = " The prototype of functions submitted to dispatch queues."] # [doc = ""] # [doc = " This is deliberately `extern \"C\"`, since libdispatch doesn't support"] # [doc = " unwinding in handler functions, and this gives us better error messages"] # [doc = " if that does happen."] # [allow (non_camel_case_types)] pub type dispatch_function_t = extern "C" fn (* mut core :: ffi :: c_void) ;
};
}
