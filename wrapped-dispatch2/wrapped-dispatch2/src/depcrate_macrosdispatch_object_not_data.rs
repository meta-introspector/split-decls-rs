// Generated macro for dispatch_object_not_data (macro)
macro_rules! Depcrate_macrosdispatch_object_not_data {
() => {
// Module: crate::macros
// Provides: {"dispatch_object_not_data"}
// Dependencies: {}
macro_rules ! dispatch_object_not_data { (unsafe $ type : ident) => { unsafe impl Send for $ type { } unsafe impl Sync for $ type { } } ; }
};
}
