// Generated macro for simd_impl (macro)
macro_rules! Depcrate_distr_utilssimd_impl {
() => {
// Module: crate::distr::utils
// Provides: {"simd_impl"}
// Dependencies: {}
# [cfg (feature = "simd_support")] macro_rules ! simd_impl { ($ fty : ident , $ uty : ident) => { impl < const LANES : usize > FloatSIMDUtils for Simd <$ fty , LANES > where LaneCount < LANES >: SupportedLaneCount , { type Mask = Mask <<$ fty as SimdElement >:: Mask , LANES >; type UInt = Simd <$ uty , LANES >; # [inline (always)] fn all_lt (self , other : Self) -> bool { self . simd_lt (other) . all () } # [inline (always)] fn all_le (self , other : Self) -> bool { self . simd_le (other) . all () } # [inline (always)] fn all_finite (self) -> bool { self . is_finite () . all () } # [inline (always)] fn gt_mask (self , other : Self) -> Self :: Mask { self . simd_gt (other) } # [inline (always)] fn decrease_masked (self , mask : Self :: Mask) -> Self { debug_assert ! (mask . any () , "At least one lane must be set") ; Self :: from_bits (self . to_bits () + mask . to_int () . cast ()) } # [inline] fn cast_from_int (i : Self :: UInt) -> Self { i . cast () } } # [cfg (test)] impl < const LANES : usize > FloatSIMDScalarUtils for Simd <$ fty , LANES > where LaneCount < LANES >: SupportedLaneCount , { type Scalar = $ fty ; # [inline] fn replace (mut self , index : usize , new_value : Self :: Scalar) -> Self { self . as_mut_array () [index] = new_value ; self } # [inline] fn extract_lane (self , index : usize) -> Self :: Scalar { self . as_array () [index] } } } ; }
};
}
