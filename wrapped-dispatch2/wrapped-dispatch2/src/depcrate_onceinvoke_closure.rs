// Generated macro for invoke_closure (function)
macro_rules! Depcrate_onceinvoke_closure {
() => {
// Module: crate::once
// Provides: {"invoke_closure"}
// Dependencies: {}
extern "C" fn invoke_closure < F > (context : * mut c_void) where F : FnOnce () , { let context : * mut Option < F > = context . cast () ; let closure : & mut Option < F > = unsafe { & mut * context } ; let closure = unsafe { closure . take () . unwrap_unchecked () } ; (closure) () ; }
};
}
