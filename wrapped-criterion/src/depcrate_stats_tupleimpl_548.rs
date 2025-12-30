// Generated macro for impl_548 (impl)
macro_rules! Depcrate_stats_tupleimpl_548 {
() => {
// Module: crate::stats::tuple
// Provides: {"impl_548"}
// Dependencies: {}
impl < A , B > TupledDistributionsBuilder for (Vec < A > , Vec < B >) where A : Copy , B : Copy , { type Item = (A , B) ; fn new (size : usize) -> (Vec < A > , Vec < B >) { (Vec :: with_capacity (size) , Vec :: with_capacity (size)) } fn push (& mut self , tuple : (A , B)) { (self . 0) . push (tuple . 0) ; (self . 1) . push (tuple . 1) ; } fn extend (& mut self , other : & mut (Vec < A > , Vec < B >)) { (self . 0) . append (& mut other . 0) ; (self . 1) . append (& mut other . 1) ; } fn complete (self) -> (Distribution < A > , Distribution < B >) { (Distribution (self . 0 . into_boxed_slice ()) , Distribution (self . 1 . into_boxed_slice ()) ,) } }
};
}
