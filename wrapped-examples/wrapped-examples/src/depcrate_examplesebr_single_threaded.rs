// Generated macro for ebr_single_threaded (function)
macro_rules! Depcrate_examplesebr_single_threaded {
() => {
// Module: crate::examples
// Provides: {"ebr_single_threaded"}
// Dependencies: {}
# [test] fn ebr_single_threaded () { static DROP_CNT : AtomicIsize = AtomicIsize :: new (0) ; let r = Shared :: new (R (& DROP_CNT)) ; let guard = Guard :: new () ; let p = r . get_guarded_ptr (& guard) ; assert_eq ! (DROP_CNT . load (Relaxed) , 0) ; drop (r) ; assert_eq ! (DROP_CNT . load (Relaxed) , 0) ; assert ! (p . as_ref () . is_some ()) ; drop (guard) ; while DROP_CNT . load (Relaxed) != 1 { Guard :: new () . accelerate () ; yield_now () ; } assert_eq ! (DROP_CNT . load (Relaxed) , 1) ; }
};
}
