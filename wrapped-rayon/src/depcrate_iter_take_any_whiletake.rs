// Generated macro for take (function)
macro_rules! Depcrate_iter_take_any_whiletake {
() => {
// Module: crate::iter::take_any_while
// Provides: {"take"}
// Dependencies: {}
fn take < T > (item : & T , taking : & AtomicBool , predicate : & impl Fn (& T) -> bool) -> bool { if ! taking . load (Ordering :: Relaxed) { return false ; } if predicate (item) { return true ; } taking . store (false , Ordering :: Relaxed) ; false }
};
}
