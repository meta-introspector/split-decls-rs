// Generated macro for sqrt_tonelli_shanks (function)
macro_rules! Depcrate_helperssqrt_tonelli_shanks {
() => {
// Module: crate::helpers
// Provides: {"sqrt_tonelli_shanks"}
// Dependencies: {}
# [doc = " Constant-time implementation of Tonelli–Shanks' square-root algorithm for"] # [doc = " `p mod 16 = 1`."] # [doc = ""] # [doc = " `tm1d2` should be set to `(t - 1) // 2`, where `t = (modulus - 1) >> F::S`."] # [doc = ""] # [doc = " ## Implementing [`Field::sqrt`]"] # [doc = ""] # [doc = " This function can be used to implement [`Field::sqrt`] for fields that both implement"] # [doc = " [`PrimeField`] and satisfy `p mod 16 = 1`."] # [doc = ""] # [doc = " [`Field::sqrt`]: crate::Field::sqrt"] pub fn sqrt_tonelli_shanks < F : PrimeField , S : AsRef < [u64] > > (f : & F , tm1d2 : S) -> CtOption < F > { let w = f . pow_vartime (tm1d2) ; let mut v = F :: S ; let mut x = w * f ; let mut b = x * w ; let mut z = F :: ROOT_OF_UNITY ; for max_v in (1 ..= F :: S) . rev () { let mut k = 1 ; let mut b2k = b . square () ; let mut j_less_than_v : Choice = 1 . into () ; for j in 2 .. max_v { let b2k_is_one = b2k . ct_eq (& F :: ONE) ; let squared = F :: conditional_select (& b2k , & z , b2k_is_one) . square () ; b2k = F :: conditional_select (& squared , & b2k , b2k_is_one) ; let new_z = F :: conditional_select (& z , & squared , b2k_is_one) ; j_less_than_v &= ! j . ct_eq (& v) ; k = u32 :: conditional_select (& j , & k , b2k_is_one) ; z = F :: conditional_select (& z , & new_z , j_less_than_v) ; } let result = x * z ; x = F :: conditional_select (& result , & x , b . ct_eq (& F :: ONE)) ; z = z . square () ; b *= z ; v = k ; } CtOption :: new (x , (x * x) . ct_eq (f) ,) }
};
}
