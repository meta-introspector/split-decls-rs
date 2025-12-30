// Generated macro for Integer (trait)
macro_rules! Depcrate_lexical_numInteger {
() => {
// Module: crate::lexical::num
// Provides: {"Integer"}
// Dependencies: {}
# [doc = " Defines a trait that supports integral operations."] pub trait Integer : Number + ops :: BitAnd < Output = Self > + ops :: Shr < i32 , Output = Self > { const ZERO : Self ; }
};
}
