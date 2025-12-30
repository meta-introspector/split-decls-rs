// Generated macro for bin_op_simd_float_all (function)
macro_rules! Depcrate_shims_x86bin_op_simd_float_all {
() => {
// Module: crate::shims::x86
// Provides: {"bin_op_simd_float_all"}
// Dependencies: {}
# [doc = " Performs `which` operation on each component of `left` and"] # [doc = " `right`, storing the result is stored in `dest`."] fn bin_op_simd_float_all < 'tcx , F : rustc_apfloat :: Float > (ecx : & mut crate :: MiriInterpCx < 'tcx > , which : FloatBinOp , left : & OpTy < 'tcx > , right : & OpTy < 'tcx > , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , () > { let (left , left_len) = ecx . project_to_simd (left) ? ; let (right , right_len) = ecx . project_to_simd (right) ? ; let (dest , dest_len) = ecx . project_to_simd (dest) ? ; assert_eq ! (dest_len , left_len) ; assert_eq ! (dest_len , right_len) ; for i in 0 .. dest_len { let left = ecx . read_immediate (& ecx . project_index (& left , i) ?) ? ; let right = ecx . read_immediate (& ecx . project_index (& right , i) ?) ? ; let dest = ecx . project_index (& dest , i) ? ; let res = bin_op_float :: < F > (which , & left , & right) ? ; ecx . write_scalar (res , & dest) ? ; } interp_ok (()) }
};
}
