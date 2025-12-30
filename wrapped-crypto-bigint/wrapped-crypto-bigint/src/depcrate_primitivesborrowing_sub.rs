// Generated macro for borrowing_sub (function)
macro_rules! Depcrate_primitivesborrowing_sub {
() => {
// Module: crate::primitives
// Provides: {"borrowing_sub"}
// Dependencies: {}
# [doc = " Computes `lhs - (rhs + borrow)`, returning the result along with the new borrow."] # [inline (always)] pub (crate) const fn borrowing_sub (lhs : Word , rhs : Word , borrow : Word) -> (Word , Word) { let a = lhs as WideWord ; let b = rhs as WideWord ; let borrow = (borrow >> (Word :: BITS - 1)) as WideWord ; let ret = a . wrapping_sub (b + borrow) ; (ret as Word , (ret >> Word :: BITS) as Word) }
};
}
