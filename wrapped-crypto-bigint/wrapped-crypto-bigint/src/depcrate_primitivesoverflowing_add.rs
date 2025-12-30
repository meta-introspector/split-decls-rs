// Generated macro for overflowing_add (function)
macro_rules! Depcrate_primitivesoverflowing_add {
() => {
// Module: crate::primitives
// Provides: {"overflowing_add"}
// Dependencies: {}
# [doc = " Computes `lhs + rhs`, returning the result along with the carry (0 or 1)."] # [inline (always)] pub (crate) const fn overflowing_add (lhs : Word , rhs : Word) -> (Word , Word) { let (res , carry) = lhs . overflowing_add (rhs) ; (res , carry as Word) }
};
}
