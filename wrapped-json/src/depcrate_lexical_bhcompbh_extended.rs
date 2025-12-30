// Generated macro for bh_extended (function)
macro_rules! Depcrate_lexical_bhcompbh_extended {
() => {
// Module: crate::lexical::bhcomp
// Provides: {"bh_extended"}
// Dependencies: {}
# [doc = " Calculate `b+h` from a representation of `b` as a float."] # [inline] pub (super) fn bh_extended < F : Float > (f : F) -> ExtendedFloat { let b = b_extended (f) ; ExtendedFloat { mant : (b . mant << 1) + 1 , exp : b . exp - 1 , } }
};
}
