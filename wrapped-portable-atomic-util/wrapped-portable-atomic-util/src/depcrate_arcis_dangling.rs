// Generated macro for is_dangling (function)
macro_rules! Depcrate_arcis_dangling {
() => {
// Module: crate::arc
// Provides: {"is_dangling"}
// Dependencies: {}
fn is_dangling < T : ? Sized > (ptr : * const T) -> bool { (ptr as * const ()) . addr () == usize :: MAX }
};
}
