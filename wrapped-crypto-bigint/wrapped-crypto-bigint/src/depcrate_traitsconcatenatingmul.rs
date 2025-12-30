// Generated macro for ConcatenatingMul (trait)
macro_rules! Depcrate_traitsConcatenatingMul {
() => {
// Module: crate::traits
// Provides: {"ConcatenatingMul"}
// Dependencies: {}
# [doc = " Widening multiply: returns a value with a number of limbs equal to the sum of the inputs."] pub trait ConcatenatingMul < Rhs = Self > : Sized { # [doc = " Output of the widening multiplication."] type Output : Integer ; # [doc = " Perform widening multiplication."] fn concatenating_mul (& self , rhs : Rhs) -> Self :: Output ; }
};
}
