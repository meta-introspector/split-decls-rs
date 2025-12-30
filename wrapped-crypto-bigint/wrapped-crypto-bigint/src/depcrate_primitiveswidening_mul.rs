// Generated macro for widening_mul (function)
macro_rules! Depcrate_primitiveswidening_mul {
() => {
// Module: crate::primitives
// Provides: {"widening_mul"}
// Dependencies: {}
# [doc = " Computes `lhs * rhs`, returning the low and the high words of the result."] # [inline (always)] pub (crate) const fn widening_mul (lhs : Word , rhs : Word) -> (Word , Word) { let a = lhs as WideWord ; let b = rhs as WideWord ; let ret = a * b ; (ret as Word , (ret >> Word :: BITS) as Word) }
};
}
