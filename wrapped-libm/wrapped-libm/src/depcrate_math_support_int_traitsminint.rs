// Generated macro for MinInt (trait)
macro_rules! Depcrate_math_support_int_traitsMinInt {
() => {
// Module: crate::math::support::int_traits
// Provides: {"MinInt"}
// Dependencies: {}
# [doc = " Minimal integer implementations needed on all integer types, including wide integers."] # [allow (dead_code)] pub trait MinInt : Copy + fmt :: Debug + ops :: BitOr < Output = Self > + ops :: Not < Output = Self > + ops :: Shl < u32 , Output = Self > { # [doc = " Type with the same width but other signedness"] type OtherSign : MinInt ; # [doc = " Unsigned version of Self"] type Unsigned : MinInt ; # [doc = " If `Self` is a signed integer"] const SIGNED : bool ; # [doc = " The bitwidth of the int type"] const BITS : u32 ; const ZERO : Self ; const ONE : Self ; const MIN : Self ; const MAX : Self ; }
};
}
