// Generated macro for function_wrapper (function)
macro_rules! Depcrate_utilsfunction_wrapper {
() => {
// Module: crate::utils
// Provides: {"function_wrapper"}
// Dependencies: {}
pub (crate) extern "C" fn function_wrapper < F > (work_boxed : * mut c_void) where F : FnOnce () , { let work = unsafe { Box :: from_raw (work_boxed . cast :: < F > ()) } ; (* work) () ; }
};
}
