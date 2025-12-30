// Generated macro for impl_31 (impl)
macro_rules! Depcrate_rt_allocimpl_31 {
() => {
// Module: crate::rt::alloc
// Provides: {"impl_31"}
// Dependencies: {}
impl Allocation { pub (crate) fn new (location : Location) -> Allocation { rt :: execution (| execution | { let state = execution . objects . insert (State { is_dropped : false , allocated : location , }) ; trace ! (? state , % location , "Allocation::new") ; Allocation { state } }) } }
};
}
