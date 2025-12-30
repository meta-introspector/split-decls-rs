// Generated macro for alloc (function)
macro_rules! Depcrate_rt_allocalloc {
() => {
// Module: crate::rt::alloc
// Provides: {"alloc"}
// Dependencies: {}
# [doc = " Track a raw allocation"] pub (crate) fn alloc (ptr : * mut u8 , location : Location) { rt :: execution (| execution | { let state = execution . objects . insert (State { is_dropped : false , allocated : location , }) ; let allocation = Allocation { state } ; trace ! (? allocation . state , ? ptr , % location , "alloc") ; let prev = execution . raw_allocations . insert (ptr as usize , allocation) ; assert ! (prev . is_none () , "pointer already tracked") ; }) ; }
};
}
