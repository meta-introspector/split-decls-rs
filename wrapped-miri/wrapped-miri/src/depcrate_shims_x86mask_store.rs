// Generated macro for mask_store (function)
macro_rules! Depcrate_shims_x86mask_store {
() => {
// Module: crate::shims::x86
// Provides: {"mask_store"}
// Dependencies: {}
# [doc = " Conditionally stores into `ptr` according the high bit of each"] # [doc = " element of `mask`. `ptr` does not need to be aligned."] fn mask_store < 'tcx > (ecx : & mut crate :: MiriInterpCx < 'tcx > , ptr : & OpTy < 'tcx > , mask : & OpTy < 'tcx > , value : & OpTy < 'tcx > ,) -> InterpResult < 'tcx , () > { let (mask , mask_len) = ecx . project_to_simd (mask) ? ; let (value , value_len) = ecx . project_to_simd (value) ? ; assert_eq ! (value_len , mask_len) ; let mask_item_size = mask . layout . field (ecx , 0) . size ; let high_bit_offset = mask_item_size . bits () . strict_sub (1) ; let ptr = ecx . read_pointer (ptr) ? ; for i in 0 .. value_len { let mask = ecx . project_index (& mask , i) ? ; let value = ecx . project_index (& value , i) ? ; if ecx . read_scalar (& mask) ? . to_uint (mask_item_size) ? >> high_bit_offset != 0 { let ptr = ptr . wrapping_offset (value . layout . size * i , & ecx . tcx) ; let dest = ecx . ptr_to_mplace_unaligned (ptr , value . layout) ; ecx . copy_op (& value , & dest) ? ; } } interp_ok (()) }
};
}
