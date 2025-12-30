// Generated macro for skip (function)
macro_rules! Depcrate_iter_skip_any_whileskip {
() => {
// Module: crate::iter::skip_any_while
// Provides: {"skip"}
// Dependencies: {}
fn skip < T > (item : & T , skipping : & AtomicBool , predicate : & impl Fn (& T) -> bool) -> bool { if ! skipping . load (Ordering :: Relaxed) { return false ; } if predicate (item) { return true ; } skipping . store (false , Ordering :: Relaxed) ; false }
};
}
