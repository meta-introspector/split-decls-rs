// Generated macro for DFloat (trait)
macro_rules! Depcrate_math_support_float_traitsDFloat {
() => {
// Module: crate::math::support::float_traits
// Provides: {"DFloat"}
// Dependencies: {}
# [doc = " Trait for floats twice the bit width of another integer."] pub trait DFloat : Float { # [doc = " Float that is half the bit width of the floatthis trait is implemented for."] type H : HFloat < D = Self > ; # [doc = " Narrow the float type."] fn narrow (self) -> Self :: H ; }
};
}
