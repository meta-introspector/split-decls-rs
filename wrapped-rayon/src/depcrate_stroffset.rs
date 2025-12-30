// Generated macro for offset (function)
macro_rules! Depcrate_stroffset {
() => {
// Module: crate::str
// Provides: {"offset"}
// Dependencies: {}
# [inline] fn offset < T > (base : usize) -> impl Fn ((usize , T)) -> (usize , T) { move | (i , x) | (base + i , x) }
};
}
