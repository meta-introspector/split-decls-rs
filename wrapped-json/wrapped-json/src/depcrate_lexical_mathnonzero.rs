// Generated macro for nonzero (function)
macro_rules! Depcrate_lexical_mathnonzero {
() => {
// Module: crate::lexical::math
// Provides: {"nonzero"}
// Dependencies: {}
# [doc = " Check if any of the remaining bits are non-zero."] # [inline] pub fn nonzero < T : Integer > (x : & [T] , rindex : usize) -> bool { let len = x . len () ; let slc = & x [.. len - rindex] ; slc . iter () . rev () . any (| & x | x != T :: ZERO) }
};
}
