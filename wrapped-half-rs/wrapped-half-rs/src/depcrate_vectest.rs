// Generated macro for test (module)
macro_rules! Depcrate_vectest {
() => {
// Module: crate::vec
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: { HalfBitsVecExt , HalfFloatVecExt } ; use crate :: { bf16 , f16 } ; # [cfg (all (feature = "alloc" , not (feature = "std")))] use alloc :: vec ; # [test] fn test_vec_conversions_f16 () { let numbers = vec ! [f16 :: E , f16 :: PI , f16 :: EPSILON , f16 :: FRAC_1_SQRT_2] ; let bits = vec ! [f16 :: E . to_bits () , f16 :: PI . to_bits () , f16 :: EPSILON . to_bits () , f16 :: FRAC_1_SQRT_2 . to_bits () ,] ; let bits_cloned = bits . clone () ; let from_bits = bits . reinterpret_into :: < f16 > () ; assert_eq ! (& from_bits [..] , & numbers [..]) ; let to_bits = from_bits . reinterpret_into () ; assert_eq ! (& to_bits [..] , & bits_cloned [..]) ; } # [test] fn test_vec_conversions_bf16 () { let numbers = vec ! [bf16 :: E , bf16 :: PI , bf16 :: EPSILON , bf16 :: FRAC_1_SQRT_2] ; let bits = vec ! [bf16 :: E . to_bits () , bf16 :: PI . to_bits () , bf16 :: EPSILON . to_bits () , bf16 :: FRAC_1_SQRT_2 . to_bits () ,] ; let bits_cloned = bits . clone () ; let from_bits = bits . reinterpret_into :: < bf16 > () ; assert_eq ! (& from_bits [..] , & numbers [..]) ; let to_bits = from_bits . reinterpret_into () ; assert_eq ! (& to_bits [..] , & bits_cloned [..]) ; } }
};
}
