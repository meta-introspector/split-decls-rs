// Generated macro for PAGE_COUNT (static)
macro_rules! Depcrate_shims_native_lib_trace_parentPAGE_COUNT {
() => {
// Module: crate::shims::native_lib::trace::parent
// Provides: {"PAGE_COUNT"}
// Dependencies: {}
# [doc = " How many consecutive pages to unprotect. 1 by default, unlikely to be set"] # [doc = " higher than 2."] static PAGE_COUNT : AtomicUsize = AtomicUsize :: new (1) ;
};
}
