// Generated macro for impl_1218 (impl)
macro_rules! Depcrate_units_ratioimpl_1218 {
() => {
// Module: crate::units::ratio
// Provides: {"impl_1218"}
// Dependencies: {}
impl IcuRatio { # [doc = " Creates a new `IcuRatio` from the given numerator and denominator."] pub fn from_big_ints (numerator : BigInt , denominator : BigInt) -> Self { Self (Ratio :: new (numerator , denominator)) } # [doc = " Returns the current [`IcuRatio`] as a [`Ratio`] of [`BigInt`]."] pub fn get_ratio (self) -> Ratio < BigInt > { self . 0 } # [doc = " Creates a new `IcuRatio` from the given integer."] pub fn from_integer (value : u64) -> Self { Self (Ratio :: from_integer (value . into ())) } # [doc = " Returns the reciprocal of the ratio."] # [doc = " For example, the reciprocal of 2/3 is 3/2."] pub (crate) fn recip (& self) -> Self { Self (self . 0 . recip ()) } # [doc = " Returns the absolute value of the ratio."] pub fn abs (& self) -> Self { Self (self . 0 . abs ()) } # [doc = " Returns a Ratio with value of 2."] pub fn two () -> Self { Self (Ratio :: from_integer (2 . into ())) } # [doc = " Returns a Ratio with value of 10."] pub fn ten () -> Self { Self (Ratio :: from_integer (10 . into ())) } # [doc = " Returns true if the ratio is negative."] pub fn is_negative (& self) -> bool { self . 0 . is_negative () } }
};
}
