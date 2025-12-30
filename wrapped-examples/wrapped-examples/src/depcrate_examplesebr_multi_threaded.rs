// Generated macro for ebr_multi_threaded (function)
macro_rules! Depcrate_examplesebr_multi_threaded {
() => {
// Module: crate::examples
// Provides: {"ebr_multi_threaded"}
// Dependencies: {}
# [test] fn ebr_multi_threaded () { static DROP_CNT : AtomicIsize = AtomicIsize :: new (0) ; let r1 = Owned :: new (R (& DROP_CNT)) ; let r2 = AtomicShared :: new (R (& DROP_CNT)) ; thread :: scope (| s | { s . spawn (| | { let guard = Guard :: new () ; let p = r1 . get_guarded_ptr (& guard) ; drop (r1) ; assert ! (p . as_ref () . unwrap () . 0 . load (Relaxed) <= 1) ; }) ; s . spawn (| | { let guard = Guard :: new () ; let p = r2 . load (Acquire , & guard) ; assert ! (p . as_ref () . unwrap () . 0 . load (Relaxed) <= 1) ; let r3 = r2 . get_shared (Acquire , & guard) . unwrap () ; assert ! (r3 . 0 . load (Relaxed) <= 1) ; let r4 = r2 . compare_exchange (p , (None , Tag :: None) , Acquire , Relaxed , & guard) . ok () . unwrap () . 0 . unwrap () ; assert ! (r4 . 0 . load (Relaxed) <= 1) ; }) ; }) ; while DROP_CNT . load (Relaxed) != 2 { Guard :: new () . accelerate () ; yield_now () ; } assert_eq ! (DROP_CNT . load (Relaxed) , 2) ; }
};
}
