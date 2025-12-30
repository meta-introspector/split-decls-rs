// Generated macro for pmulhrsw (function)
macro_rules! Depcrate_shims_x86pmulhrsw {
() => {
// Module: crate::shims::x86
// Provides: {"pmulhrsw"}
// Dependencies: {}
# [doc = " Multiplies packed 16-bit signed integer values, truncates the 32-bit"] # [doc = " product to the 18 most significant bits by right-shifting, and then"] # [doc = " divides the 18-bit value by 2 (rounding to nearest) by first adding"] # [doc = " 1 and then taking the bits `1..=16`."] # [doc = ""] # [doc = " <https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_mulhrs_epi16>"] # [doc = " <https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_mulhrs_epi16>"] fn pmulhrsw < 'tcx > (ecx : & mut crate :: MiriInterpCx < 'tcx > , left : & OpTy < 'tcx > , right : & OpTy < 'tcx > , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , () > { let (left , left_len) = ecx . project_to_simd (left) ? ; let (right , right_len) = ecx . project_to_simd (right) ? ; let (dest , dest_len) = ecx . project_to_simd (dest) ? ; assert_eq ! (dest_len , left_len) ; assert_eq ! (dest_len , right_len) ; for i in 0 .. dest_len { let left = ecx . read_scalar (& ecx . project_index (& left , i) ?) ? . to_i16 () ? ; let right = ecx . read_scalar (& ecx . project_index (& right , i) ?) ? . to_i16 () ? ; let dest = ecx . project_index (& dest , i) ? ; let res = (i32 :: from (left) . strict_mul (right . into ()) >> 14) . strict_add (1) >> 1 ; # [expect (clippy :: as_conversions)] let res = res as i16 ; ecx . write_scalar (Scalar :: from_i16 (res) , & dest) ? ; } interp_ok (()) }
};
}
