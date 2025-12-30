// Generated macro for Primitive (trait)
macro_rules! Depcrate_traitsPrimitive {
() => {
// Module: crate::traits
// Provides: {"Primitive"}
// Dependencies: {}
# [doc = " The type of each channel in a pixel. For example, this can be `u8`, `u16`, `f32`."] pub trait Primitive : Copy + NumCast + Num + PartialOrd < Self > + Clone + Bounded { # [doc = " The maximum value for this type of primitive within the context of color."] # [doc = " For floats, the maximum is `1.0`, whereas the integer types inherit their usual maximum values."] const DEFAULT_MAX_VALUE : Self ; # [doc = " The minimum value for this type of primitive within the context of color."] # [doc = " For floats, the minimum is `0.0`, whereas the integer types inherit their usual minimum values."] const DEFAULT_MIN_VALUE : Self ; }
};
}
