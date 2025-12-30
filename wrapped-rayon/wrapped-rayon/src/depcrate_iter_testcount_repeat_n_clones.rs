// Generated macro for count_repeat_n_clones (function)
macro_rules! Depcrate_iter_testcount_repeat_n_clones {
() => {
// Module: crate::iter::test
// Provides: {"count_repeat_n_clones"}
// Dependencies: {}
# [test] fn count_repeat_n_clones () { use std :: sync :: atomic :: { AtomicUsize , Ordering } ; static CLONES : AtomicUsize = AtomicUsize :: new (0) ; static DROPS : AtomicUsize = AtomicUsize :: new (0) ; struct Counter ; impl Clone for Counter { fn clone (& self) -> Self { CLONES . fetch_add (1 , Ordering :: Relaxed) ; Counter } } impl Drop for Counter { fn drop (& mut self) { DROPS . fetch_add (1 , Ordering :: Relaxed) ; } } # [track_caller] fn check (clones : usize , drops : usize) { assert_eq ! (CLONES . swap (0 , Ordering :: Relaxed) , clones , "clones") ; assert_eq ! (DROPS . swap (0 , Ordering :: Relaxed) , drops , "drops") ; } drop (repeat_n (Counter , 100)) ; check (0 , 1) ; let empty = repeat_n (Counter , 0) ; check (0 , 1) ; let empty2 = empty . clone () ; check (0 , 0) ; assert_eq ! (empty . count () , 0) ; assert_eq ! (empty2 . count () , 0) ; check (0 , 0) ; let par_iter = repeat_n (Counter , 100) ; let par_iter2 = par_iter . clone () ; check (1 , 0) ; assert_eq ! (par_iter . count () , 100) ; check (99 , 100) ; assert_eq ! (par_iter2 . map (std :: mem :: forget) . count () , 100) ; check (99 , 0) ; let step99 = repeat_n (Counter , 100) . step_by (99) ; assert_eq ! (step99 . count () , 2) ; check (2 , 3) ; let step99 = repeat_n (Counter , 100) . step_by (99) . with_min_len (2) ; assert_eq ! (step99 . count () , 2) ; check (1 , 2) ; let step50 = repeat_n (Counter , 100) . step_by (50) ; assert_eq ! (step50 . count () , 2) ; check (3 , 4) ; }
};
}
