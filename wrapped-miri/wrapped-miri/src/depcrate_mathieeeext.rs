// Generated macro for IeeeExt (trait)
macro_rules! Depcrate_mathIeeeExt {
() => {
// Module: crate::math
// Provides: {"IeeeExt"}
// Dependencies: {}
# [doc = " Extend functionality of `rustc_apfloat` softfloats for IEEE float types."] pub trait IeeeExt : rustc_apfloat :: Float { # [inline] fn one () -> Self { Self :: from_u128 (1) . value } # [inline] fn two () -> Self { Self :: from_u128 (2) . value } # [inline] fn three () -> Self { Self :: from_u128 (3) . value } fn pi () -> Self ; # [inline] fn clamp (self , min : Self , max : Self) -> Self { self . maximum (min) . minimum (max) } }
};
}
