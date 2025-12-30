// Generated macro for PAGE_ADDR (static)
macro_rules! Depcrate_shims_native_lib_trace_parentPAGE_ADDR {
() => {
// Module: crate::shims::native_lib::trace::parent
// Provides: {"PAGE_ADDR"}
// Dependencies: {}
# [doc = " The address of the page set to be edited, initialised to a sentinel null"] # [doc = " pointer."] static PAGE_ADDR : AtomicPtr < u8 > = AtomicPtr :: new (std :: ptr :: null_mut ()) ;
};
}
