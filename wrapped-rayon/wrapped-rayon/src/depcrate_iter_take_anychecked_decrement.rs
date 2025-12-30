// Generated macro for checked_decrement (function)
macro_rules! Depcrate_iter_take_anychecked_decrement {
() => {
// Module: crate::iter::take_any
// Provides: {"checked_decrement"}
// Dependencies: {}
fn checked_decrement (u : & AtomicUsize) -> bool { u . fetch_update (Ordering :: Relaxed , Ordering :: Relaxed , | u | u . checked_sub (1)) . is_ok () }
};
}
