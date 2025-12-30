// Generated macro for impl_13 (impl)
macro_rules! Depcrate_operandsimpl_13 {
() => {
// Module: crate::operands
// Provides: {"impl_13"}
// Dependencies: {}
impl PluralOperands { fn from_significand_and_exponent (dec : & Decimal , exp : u8) -> PluralOperands { let exp_i16 = i16 :: from (exp) ; let mag_range = dec . absolute . magnitude_range () ; let mag_high = core :: cmp :: min (17 , * mag_range . end () + exp_i16) ; let mag_low = core :: cmp :: max (- 18 , * mag_range . start () + exp_i16) ; let mut i : u64 = 0 ; for magnitude in (0 ..= mag_high) . rev () { i *= 10 ; i += dec . absolute . digit_at (magnitude - exp_i16) as u64 ; } let mut f : u64 = 0 ; let mut t : u64 = 0 ; let mut w : usize = 0 ; for magnitude in (mag_low ..= - 1) . rev () { let digit = dec . absolute . digit_at (magnitude - exp_i16) as u64 ; f *= 10 ; f += digit ; if digit != 0 { t = f ; w = (- magnitude) as usize ; } } Self { i , v : (- mag_low) as usize , w , f , t , c : usize :: from (exp) , } } # [doc = " Whether these [`PluralOperands`] are exactly the number 0, which might be a special case."] pub (crate) fn is_exactly_zero (self) -> bool { self == Self { i : 0 , v : 0 , w : 0 , f : 0 , t : 0 , c : 0 , } } # [doc = " Whether these [`PluralOperands`] are exactly the number 1, which might be a special case."] pub (crate) fn is_exactly_one (self) -> bool { self == Self { i : 1 , v : 0 , w : 0 , f : 0 , t : 0 , c : 0 , } } }
};
}
