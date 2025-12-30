// Generated macro for new_mpfloat (function)
macro_rules! Depcrate_mpfloatnew_mpfloat {
() => {
// Module: crate::mpfloat
// Provides: {"new_mpfloat"}
// Dependencies: {}
# [doc = " Create a multiple-precision float with the correct number of bits for a concrete float type."] fn new_mpfloat < F : Float > () -> MpFloat { MpFloat :: new (F :: SIG_BITS + 1) }
};
}
