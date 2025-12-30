// Generated macro for scope_mix (function)
macro_rules! Depcrate_iter_testscope_mix {
() => {
// Module: crate::iter::test
// Provides: {"scope_mix"}
// Dependencies: {}
# [test] fn scope_mix () { let counter_p = & AtomicUsize :: new (0) ; scope (| s | { s . spawn (move | s | { divide_and_conquer (s , counter_p , 1024) ; }) ; s . spawn (move | _ | { let a : Vec < i32 > = (0 .. 1024) . collect () ; let r1 = a . par_iter () . map (| & i | i + 1) . reduce_with (| i , j | i + j) ; let r2 = a . iter () . map (| & i | i + 1) . sum () ; assert_eq ! (r1 . unwrap () , r2) ; }) ; }) ; }
};
}
