// Generated macro for AsCast (trait)
macro_rules! Depcrate_lexical_numAsCast {
() => {
// Module: crate::lexical::num
// Provides: {"AsCast"}
// Dependencies: {}
# [doc = " An interface for casting between machine scalars."] pub trait AsCast : AsPrimitive { # [doc = " Creates a number from another value that can be converted into"] # [doc = " a primitive via the `AsPrimitive` trait."] fn as_cast < N : AsPrimitive > (n : N) -> Self ; }
};
}
