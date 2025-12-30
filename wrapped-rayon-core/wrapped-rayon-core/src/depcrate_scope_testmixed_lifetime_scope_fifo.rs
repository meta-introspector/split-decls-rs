// Generated macro for mixed_lifetime_scope_fifo (function)
macro_rules! Depcrate_scope_testmixed_lifetime_scope_fifo {
() => {
// Module: crate::scope::test
// Provides: {"mixed_lifetime_scope_fifo"}
// Dependencies: {}
# [test] fn mixed_lifetime_scope_fifo () { fn increment < 'slice , 'counter > (counters : & 'slice [& 'counter AtomicUsize]) { scope_fifo (move | s : & ScopeFifo < 'counter > | { for & c in counters { s . spawn_fifo (move | _ | { c . fetch_add (1 , Ordering :: Relaxed) ; }) ; } }) ; } let counter = AtomicUsize :: new (0) ; increment (& [& counter ; 100]) ; assert_eq ! (counter . into_inner () , 100) ; }
};
}
