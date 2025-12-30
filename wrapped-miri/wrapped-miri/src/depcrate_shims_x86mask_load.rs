// Generated macro for mask_load (function)
macro_rules! Depcrate_shims_x86mask_load {
() => {
// Module: crate::shims::x86
// Provides: {"mask_load"}
// Dependencies: {}
# [doc = " Conditionally loads from `ptr` according the high bit of each"] # [doc = " element of `mask`. `ptr` does not need to be aligned."] fn mask_load < 'tcx > (ecx : & mut crate :: MiriInterpCx < 'tcx > , ptr : & OpTy < 'tcx > , mask : & OpTy < 'tcx > , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , () > { let (mask , mask_len) = ecx . project_to_simd (mask) ? ; let (dest , dest_len) = ecx . project_to_simd (dest) ? ; assert_eq ! (dest_len , mask_len) ; let mask_item_size = mask . layout . field (ecx , 0) . size ; let high_bit_offset = mask_item_size . bits () . strict_sub (1) ; let ptr = ecx . read_pointer (ptr) ? ; for i in 0 .. dest_len { let mask = ecx . project_index (& mask , i) ? ; let dest = ecx . project_index (& dest , i) ? ; if ecx . read_scalar (& mask) ? . to_uint (mask_item_size) ? >> high_bit_offset != 0 { let ptr = ptr . wrapping_offset (dest . layout . size * i , & ecx . tcx) ; ecx . mem_copy (ptr , dest . ptr () , dest . layout . size , true) ? ; } else { ecx . write_scalar (Scalar :: from_int (0 , dest . layout . size) , & dest) ? ; } } interp_ok (()) }
};
}
