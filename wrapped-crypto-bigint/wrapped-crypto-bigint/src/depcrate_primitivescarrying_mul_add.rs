// Generated macro for carrying_mul_add (function)
macro_rules! Depcrate_primitivescarrying_mul_add {
() => {
// Module: crate::primitives
// Provides: {"carrying_mul_add"}
// Dependencies: {}
# [doc = " Computes `(lhs * rhs) + addend + carry`, returning the result along with the new carry."] # [inline (always)] pub (crate) const fn carrying_mul_add (lhs : Word , rhs : Word , addend : Word , carry : Word ,) -> (Word , Word) { let lhs = lhs as WideWord ; let rhs = rhs as WideWord ; let addend = addend as WideWord ; let carry = carry as WideWord ; let ret = ((lhs * rhs) + addend) + carry ; (ret as Word , (ret >> Word :: BITS) as Word) }
};
}
