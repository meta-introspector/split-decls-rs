// Generated macro for AsPrimitive (trait)
macro_rules! Depcrate_castAsPrimitive {
() => {
// Module: crate::cast
// Provides: {"AsPrimitive"}
// Dependencies: {}
# [doc = " A generic interface for casting between machine scalars with the"] # [doc = " `as` operator, which admits narrowing and precision loss."] # [doc = " Implementers of this trait `AsPrimitive` should behave like a primitive"] # [doc = " numeric type (e.g. a newtype around another primitive), and the"] # [doc = " intended conversion must never fail."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use num_traits::AsPrimitive;"] # [doc = " let three: i32 = (3.14159265f32).as_();"] # [doc = " assert_eq!(three, 3);"] # [doc = " ```"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " **In Rust versions before 1.45.0**, some uses of the `as` operator were not entirely safe."] # [doc = " In particular, it was undefined behavior if"] # [doc = " a truncated floating point value could not fit in the target integer"] # [doc = " type ([#10184](https://github.com/rust-lang/rust/issues/10184))."] # [doc = ""] # [doc = " ```ignore"] # [doc = " # use num_traits::AsPrimitive;"] # [doc = " let x: u8 = (1.04E+17).as_(); // UB"] # [doc = " ```"] # [doc = ""] pub trait AsPrimitive < T > : 'static + Copy where T : 'static + Copy , { # [doc = " Convert a value to another, using the `as` operator."] fn as_ (self) -> T ; }
};
}
