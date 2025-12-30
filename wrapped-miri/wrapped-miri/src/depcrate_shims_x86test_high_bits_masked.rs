// Generated macro for test_high_bits_masked (function)
macro_rules! Depcrate_shims_x86test_high_bits_masked {
() => {
// Module: crate::shims::x86
// Provides: {"test_high_bits_masked"}
// Dependencies: {}
# [doc = " Calculates two booleans."] # [doc = ""] # [doc = " The first is true when the highest bit of each element of `op & mask` is zero."] # [doc = " The second is true when the highest bit of each element of `!op & mask` is zero."] fn test_high_bits_masked < 'tcx > (ecx : & crate :: MiriInterpCx < 'tcx > , op : & OpTy < 'tcx > , mask : & OpTy < 'tcx > ,) -> InterpResult < 'tcx , (bool , bool) > { assert_eq ! (op . layout , mask . layout) ; let (op , op_len) = ecx . project_to_simd (op) ? ; let (mask , mask_len) = ecx . project_to_simd (mask) ? ; assert_eq ! (op_len , mask_len) ; let high_bit_offset = op . layout . field (ecx , 0) . size . bits () . strict_sub (1) ; let mut direct = true ; let mut negated = true ; for i in 0 .. op_len { let op = ecx . project_index (& op , i) ? ; let mask = ecx . project_index (& mask , i) ? ; let op = ecx . read_scalar (& op) ? . to_uint (op . layout . size) ? ; let mask = ecx . read_scalar (& mask) ? . to_uint (mask . layout . size) ? ; direct &= (op & mask) >> high_bit_offset == 0 ; negated &= (! op & mask) >> high_bit_offset == 0 ; } interp_ok ((direct , negated)) }
};
}
