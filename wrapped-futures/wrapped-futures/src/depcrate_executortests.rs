// Generated macro for tests (module)
macro_rules! Depcrate_executortests {
() => {
// Module: crate::executor
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: sync :: Arc ; use std :: sync :: atomic :: { AtomicUsize , Ordering } ; use super :: { Executor , Limited } ; # [test] fn limited () { fn doit (ex : Arc < Executor > , hits : Arc < AtomicUsize > , i : usize) { if i == 0 { return } hits . fetch_add (1 , Ordering :: SeqCst) ; let ex2 = ex . clone () ; ex . execute (move | | { doit (ex2 , hits , i - 1) ; }) } let n = 1_000_000 ; let hits = Arc :: new (AtomicUsize :: new (0)) ; doit (Arc :: new (Limited) , hits . clone () , n) ; assert_eq ! (hits . load (Ordering :: SeqCst) , n) ; } }
};
}
