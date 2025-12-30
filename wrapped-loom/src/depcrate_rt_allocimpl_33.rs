// Generated macro for impl_33 (impl)
macro_rules! Depcrate_rt_allocimpl_33 {
() => {
// Module: crate::rt::alloc
// Provides: {"impl_33"}
// Dependencies: {}
impl State { pub (super) fn check_for_leaks (& self , index : usize) { if ! self . is_dropped { if self . allocated . is_captured () { panic ! ("Allocation leaked.\n  Allocated: {}\n      Index: {}" , self . allocated , index) ; } else { panic ! ("Allocation leaked.\n  Index: {}" , index) ; } } } }
};
}
