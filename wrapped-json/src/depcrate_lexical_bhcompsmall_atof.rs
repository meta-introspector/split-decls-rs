// Generated macro for small_atof (function)
macro_rules! Depcrate_lexical_bhcompsmall_atof {
() => {
// Module: crate::lexical::bhcomp
// Provides: {"small_atof"}
// Dependencies: {}
# [doc = " Calculate the mantissa for a big integer with a negative exponent."] # [doc = ""] # [doc = " This invokes the comparison with `b+h`."] fn small_atof < F > (mantissa : Bigint , exponent : i32 , f : F) -> F where F : Float , { let mut real_digits = mantissa ; let real_exp = exponent ; debug_assert ! (real_exp < 0) ; let theor = bh_extended (f) ; let mut theor_digits = Bigint :: from_u64 (theor . mant) ; let theor_exp = theor . exp ; let binary_exp = theor_exp - real_exp ; let halfradix_exp = - real_exp ; let radix_exp = 0 ; if halfradix_exp != 0 { theor_digits . imul_pow5 (halfradix_exp as u32) ; } if radix_exp != 0 { theor_digits . imul_pow10 (radix_exp as u32) ; } if binary_exp > 0 { theor_digits . imul_pow2 (binary_exp as u32) ; } else if binary_exp < 0 { real_digits . imul_pow2 (- binary_exp as u32) ; } match real_digits . compare (& theor_digits) { cmp :: Ordering :: Greater => f . next_positive () , cmp :: Ordering :: Less => f , cmp :: Ordering :: Equal => f . round_positive_even () , } }
};
}
