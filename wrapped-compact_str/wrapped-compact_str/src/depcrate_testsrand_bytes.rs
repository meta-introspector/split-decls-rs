// Generated macro for rand_bytes (function)
macro_rules! Depcrate_testsrand_bytes {
() => {
// Module: crate::tests
// Provides: {"rand_bytes"}
// Dependencies: {}
# [doc = " generates a random collection of bytes, upto 80 bytes long"] pub (crate) fn rand_bytes () -> impl Strategy < Value = Vec < u8 > > { proptest :: collection :: vec (any :: < u8 > () , 0 .. 80) }
};
}
