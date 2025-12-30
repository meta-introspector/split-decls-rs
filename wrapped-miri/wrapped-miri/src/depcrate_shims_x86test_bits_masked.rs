// Generated macro for test_bits_masked (function)
macro_rules! Depcrate_shims_x86test_bits_masked {
() => {
// Module: crate::shims::x86
// Provides: {"test_bits_masked"}
// Dependencies: {}
# [doc = " Calculates two booleans."] # [doc = ""] # [doc = " The first is true when all the bits of `op & mask` are zero."] # [doc = " The second is true when `(op & mask) == mask`"] fn test_bits_masked < 'tcx > (ecx : & crate :: MiriInterpCx < 'tcx > , op : & OpTy < 'tcx > , mask : & OpTy < 'tcx > ,) -> InterpResult < 'tcx , (bool , bool) > { assert_eq ! (op . layout , mask . layout) ; let (op , op_len) = ecx . project_to_simd (op) ? ; let (mask , mask_len) = ecx . project_to_simd (mask) ? ; assert_eq ! (op_len , mask_len) ; let mut all_zero = true ; let mut masked_set = true ; for i in 0 .. op_len { let op = ecx . project_index (& op , i) ? ; let mask = ecx . project_index (& mask , i) ? ; let op = ecx . read_scalar (& op) ? . to_uint (op . layout . size) ? ; let mask = ecx . read_scalar (& mask) ? . to_uint (mask . layout . size) ? ; all_zero &= (op & mask) == 0 ; masked_set &= (op & mask) == mask ; } interp_ok ((all_zero , masked_set)) }
};
}
