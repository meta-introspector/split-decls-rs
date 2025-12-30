// Generated macro for bin_op_simd_float_first (function)
macro_rules! Depcrate_shims_x86bin_op_simd_float_first {
() => {
// Module: crate::shims::x86
// Provides: {"bin_op_simd_float_first"}
// Dependencies: {}
# [doc = " Performs `which` operation on the first component of `left` and `right`"] # [doc = " and copies the other components from `left`. The result is stored in `dest`."] fn bin_op_simd_float_first < 'tcx , F : rustc_apfloat :: Float > (ecx : & mut crate :: MiriInterpCx < 'tcx > , which : FloatBinOp , left : & OpTy < 'tcx > , right : & OpTy < 'tcx > , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , () > { let (left , left_len) = ecx . project_to_simd (left) ? ; let (right , right_len) = ecx . project_to_simd (right) ? ; let (dest , dest_len) = ecx . project_to_simd (dest) ? ; assert_eq ! (dest_len , left_len) ; assert_eq ! (dest_len , right_len) ; let res0 = bin_op_float :: < F > (which , & ecx . read_immediate (& ecx . project_index (& left , 0) ?) ? , & ecx . read_immediate (& ecx . project_index (& right , 0) ?) ? ,) ? ; ecx . write_scalar (res0 , & ecx . project_index (& dest , 0) ?) ? ; for i in 1 .. dest_len { ecx . copy_op (& ecx . project_index (& left , i) ? , & ecx . project_index (& dest , i) ?) ? ; } interp_ok (()) }
};
}
