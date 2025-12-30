// Generated macro for is_dangling (function)
macro_rules! Depcrate_rcis_dangling {
() => {
// Module: crate::rc
// Provides: {"is_dangling"}
// Dependencies: {}
pub (crate) fn is_dangling < T : ? Sized > (ptr : * const T) -> bool { (ptr . cast :: < () > ()) . addr () == usize :: MAX }
};
}
