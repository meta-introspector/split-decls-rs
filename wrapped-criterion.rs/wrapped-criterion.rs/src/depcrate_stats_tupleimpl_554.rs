// Generated macro for impl_554 (impl)
macro_rules! Depcrate_stats_tupleimpl_554 {
() => {
// Module: crate::stats::tuple
// Provides: {"impl_554"}
// Dependencies: {}
impl < A , B , C , D > TupledDistributionsBuilder for (Vec < A > , Vec < B > , Vec < C > , Vec < D >) where A : Copy , B : Copy , C : Copy , D : Copy , { type Item = (A , B , C , D) ; fn new (size : usize) -> (Vec < A > , Vec < B > , Vec < C > , Vec < D >) { (Vec :: with_capacity (size) , Vec :: with_capacity (size) , Vec :: with_capacity (size) , Vec :: with_capacity (size) ,) } fn push (& mut self , tuple : (A , B , C , D)) { (self . 0) . push (tuple . 0) ; (self . 1) . push (tuple . 1) ; (self . 2) . push (tuple . 2) ; (self . 3) . push (tuple . 3) ; } fn extend (& mut self , other : & mut (Vec < A > , Vec < B > , Vec < C > , Vec < D >)) { (self . 0) . append (& mut other . 0) ; (self . 1) . append (& mut other . 1) ; (self . 2) . append (& mut other . 2) ; (self . 3) . append (& mut other . 3) ; } fn complete (self ,) -> (Distribution < A > , Distribution < B > , Distribution < C > , Distribution < D > ,) { (Distribution (self . 0 . into_boxed_slice ()) , Distribution (self . 1 . into_boxed_slice ()) , Distribution (self . 2 . into_boxed_slice ()) , Distribution (self . 3 . into_boxed_slice ()) ,) } }
};
}
