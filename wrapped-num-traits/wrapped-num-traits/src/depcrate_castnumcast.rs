// Generated macro for NumCast (trait)
macro_rules! Depcrate_castNumCast {
() => {
// Module: crate::cast
// Provides: {"NumCast"}
// Dependencies: {}
# [doc = " An interface for casting between machine scalars."] pub trait NumCast : Sized + ToPrimitive { # [doc = " Creates a number from another value that can be converted into"] # [doc = " a primitive via the `ToPrimitive` trait. If the source value cannot be"] # [doc = " represented by the target type, then `None` is returned."] # [doc = ""] # [doc = " A value can be represented by the target type when it lies within"] # [doc = " the range of scalars supported by the target type."] # [doc = " For example, a negative integer cannot be represented by an unsigned"] # [doc = " integer type, and an `i64` with a very high magnitude might not be"] # [doc = " convertible to an `i32`."] # [doc = " On the other hand, conversions with possible precision loss or truncation"] # [doc = " are admitted, like an `f32` with a decimal part to an integer type, or"] # [doc = " even a large `f64` saturating to `f32` infinity."] fn from < T : ToPrimitive > (n : T) -> Option < Self > ; }
};
}
