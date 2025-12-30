// Generated macro for static_scope (function)
macro_rules! Depcrate_scope_teststatic_scope {
() => {
// Module: crate::scope::test
// Provides: {"static_scope"}
// Dependencies: {}
# [test] fn static_scope () { static COUNTER : AtomicUsize = AtomicUsize :: new (0) ; let mut range = 0 .. 100 ; let sum = range . clone () . sum () ; let iter = & mut range ; COUNTER . store (0 , Ordering :: Relaxed) ; scope (| s : & Scope < 'static > | { for i in iter { s . spawn (move | _ | { COUNTER . fetch_add (i , Ordering :: Relaxed) ; }) ; } }) ; assert_eq ! (COUNTER . load (Ordering :: Relaxed) , sum) ; }
};
}
