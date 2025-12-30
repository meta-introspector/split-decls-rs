// Generated macro for bh (function)
macro_rules! Depcrate_slowbh {
() => {
// Module: crate::slow
// Provides: {"bh"}
// Dependencies: {}
# [doc = " Calculate `b+h` from a a representation of `b` as a float."] # [inline] pub fn bh < F : Float > (float : F) -> ExtendedFloat { let fp = b (float) ; ExtendedFloat { mant : (fp . mant << 1) + 1 , exp : fp . exp - 1 , } }
};
}
