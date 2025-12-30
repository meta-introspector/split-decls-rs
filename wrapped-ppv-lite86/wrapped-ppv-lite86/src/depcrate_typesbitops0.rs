// Generated macro for BitOps0 (trait)
macro_rules! Depcrate_typesBitOps0 {
() => {
// Module: crate::types
// Provides: {"BitOps0"}
// Dependencies: {}
# [doc = " Ops that are independent of word size and endian"] pub trait BitOps0 : BitAnd < Output = Self > + BitOr < Output = Self > + BitXor < Output = Self > + BitXorAssign + Not < Output = Self > + AndNot < Output = Self > + Sized + Copy + Clone { }
};
}
