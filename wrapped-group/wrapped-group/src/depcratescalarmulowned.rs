// Generated macro for ScalarMulOwned (trait)
macro_rules! DepcrateScalarMulOwned {
() => {
// Module: crate
// Provides: {"ScalarMulOwned"}
// Dependencies: {}
# [doc = " A helper trait for references implementing group scalar multiplication."] pub trait ScalarMulOwned < Rhs , Output = Self > : for < 'r > ScalarMul < & 'r Rhs , Output > { }
};
}
