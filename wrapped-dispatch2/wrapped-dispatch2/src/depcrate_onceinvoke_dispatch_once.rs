// Generated macro for invoke_dispatch_once (function)
macro_rules! Depcrate_onceinvoke_dispatch_once {
() => {
// Module: crate::once
// Provides: {"invoke_dispatch_once"}
// Dependencies: {}
# [cfg_attr (any (target_arch = "x86" , target_arch = "x86_64" , target_vendor = "apple") , cold , inline (never))] fn invoke_dispatch_once < F > (predicate : NonNull < dispatch_once_t > , closure : F) where F : FnOnce () , { let mut closure = Some (closure) ; let context : * mut Option < F > = & mut closure ; let context : * mut c_void = context . cast () ; unsafe { DispatchOnce :: once_f (predicate , context , invoke_closure :: < F >) } ; }
};
}
