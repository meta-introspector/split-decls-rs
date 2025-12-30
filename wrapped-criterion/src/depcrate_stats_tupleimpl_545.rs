// Generated macro for impl_545 (impl)
macro_rules! Depcrate_stats_tupleimpl_545 {
() => {
// Module: crate::stats::tuple
// Provides: {"impl_545"}
// Dependencies: {}
impl < A > TupledDistributionsBuilder for (Vec < A > ,) where A : Copy , { type Item = (A ,) ; fn new (size : usize) -> (Vec < A > ,) { (Vec :: with_capacity (size) ,) } fn push (& mut self , tuple : (A ,)) { (self . 0) . push (tuple . 0) ; } fn extend (& mut self , other : & mut (Vec < A > ,)) { (self . 0) . append (& mut other . 0) ; } fn complete (self) -> (Distribution < A > ,) { (Distribution (self . 0 . into_boxed_slice ()) ,) } }
};
}
