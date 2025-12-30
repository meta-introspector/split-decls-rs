// Generated macro for partial_cmp (function)
macro_rules! Depcratepartial_cmp {
() => {
// Module: crate
// Provides: {"partial_cmp"}
// Dependencies: {}
# [doc = " Compare `a` and `b` using a partial ordering relation."] # [inline] pub fn partial_cmp < T : PartialOrd > (a : & T , b : & T) -> Option < Ordering > { a . partial_cmp (b) }
};
}
