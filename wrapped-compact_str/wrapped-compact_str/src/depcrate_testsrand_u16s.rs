// Generated macro for rand_u16s (function)
macro_rules! Depcrate_testsrand_u16s {
() => {
// Module: crate::tests
// Provides: {"rand_u16s"}
// Dependencies: {}
# [doc = " generates a random collection of `u16`s, upto 80 elements long"] pub (crate) fn rand_u16s () -> impl Strategy < Value = Vec < u16 > > { proptest :: collection :: vec (any :: < u16 > () , 0 .. 80) }
};
}
