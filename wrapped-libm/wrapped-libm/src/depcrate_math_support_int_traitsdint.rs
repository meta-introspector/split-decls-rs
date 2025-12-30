// Generated macro for DInt (trait)
macro_rules! Depcrate_math_support_int_traitsDInt {
() => {
// Module: crate::math::support::int_traits
// Provides: {"DInt"}
// Dependencies: {}
# [doc = " Trait for integers twice the bit width of another integer. This is implemented for all"] # [doc = " primitives except for `u8`, because there is not a smaller primitive."] pub trait DInt : MinInt { # [doc = " Integer that is half the bit width of the integer this trait is implemented for"] type H : HInt < D = Self > ; # [doc = " Returns the low half of `self`"] fn lo (self) -> Self :: H ; # [doc = " Returns the high half of `self`"] fn hi (self) -> Self :: H ; # [doc = " Returns the low and high halves of `self` as a tuple"] fn lo_hi (self) -> (Self :: H , Self :: H) { (self . lo () , self . hi ()) } # [doc = " Constructs an integer using lower and higher half parts"] # [allow (unused)] fn from_lo_hi (lo : Self :: H , hi : Self :: H) -> Self { lo . zero_widen () | hi . widen_hi () } }
};
}
