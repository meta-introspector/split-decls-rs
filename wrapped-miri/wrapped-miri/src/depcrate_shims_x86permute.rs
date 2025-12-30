// Generated macro for permute (function)
macro_rules! Depcrate_shims_x86permute {
() => {
// Module: crate::shims::x86
// Provides: {"permute"}
// Dependencies: {}
# [doc = " Shuffle 32-bit integers in `values` across lanes using the corresponding"] # [doc = " index in `indices`, and store the results in dst."] # [doc = ""] # [doc = " <https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_permutevar8x32_epi32>"] # [doc = " <https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_permutevar8x32_ps>"] # [doc = " <https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm512_permutexvar_epi32>"] fn permute < 'tcx > (ecx : & mut crate :: MiriInterpCx < 'tcx > , values : & OpTy < 'tcx > , indices : & OpTy < 'tcx > , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , () > { let (values , values_len) = ecx . project_to_simd (values) ? ; let (indices , indices_len) = ecx . project_to_simd (indices) ? ; let (dest , dest_len) = ecx . project_to_simd (dest) ? ; assert_eq ! (dest_len , values_len) ; assert_eq ! (dest_len , indices_len) ; assert ! (dest_len . is_power_of_two ()) ; let mask = u32 :: try_from (dest_len) . unwrap () . strict_sub (1) ; for i in 0 .. dest_len { let dest = ecx . project_index (& dest , i) ? ; let index = ecx . read_scalar (& ecx . project_index (& indices , i) ?) ? . to_u32 () ? ; let element = ecx . project_index (& values , (index & mask) . into ()) ? ; ecx . copy_op (& element , & dest) ? ; } interp_ok (()) }
};
}
