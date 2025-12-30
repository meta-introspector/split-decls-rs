// Generated macro for ScalarMul (trait)
macro_rules! DepcrateScalarMul {
() => {
// Module: crate
// Provides: {"ScalarMul"}
// Dependencies: {}
# [doc = " A helper trait for types implementing group scalar multiplication."] pub trait ScalarMul < Rhs , Output = Self > : Mul < Rhs , Output = Output > + MulAssign < Rhs > { }
};
}
