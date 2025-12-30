// Generated macro for divide_and_conquer_seq (function)
macro_rules! Depcrate_scope_testdivide_and_conquer_seq {
() => {
// Module: crate::scope::test
// Provides: {"divide_and_conquer_seq"}
// Dependencies: {}
fn divide_and_conquer_seq (counter : & AtomicUsize , size : usize) { if size > 1 { divide_and_conquer_seq (counter , size / 2) ; divide_and_conquer_seq (counter , size / 2) ; } else { counter . fetch_add (1 , Ordering :: SeqCst) ; } }
};
}
