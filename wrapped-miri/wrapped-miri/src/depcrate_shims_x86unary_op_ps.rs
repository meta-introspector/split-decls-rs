// Generated macro for unary_op_ps (function)
macro_rules! Depcrate_shims_x86unary_op_ps {
() => {
// Module: crate::shims::x86
// Provides: {"unary_op_ps"}
// Dependencies: {}
# [doc = " Performs `which` operation on each component of `op`, storing the"] # [doc = " result is stored in `dest`."] fn unary_op_ps < 'tcx > (ecx : & mut crate :: MiriInterpCx < 'tcx > , which : FloatUnaryOp , op : & OpTy < 'tcx > , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , () > { let (op , op_len) = ecx . project_to_simd (op) ? ; let (dest , dest_len) = ecx . project_to_simd (dest) ? ; assert_eq ! (dest_len , op_len) ; for i in 0 .. dest_len { let op = ecx . read_immediate (& ecx . project_index (& op , i) ?) ? ; let dest = ecx . project_index (& dest , i) ? ; let res = unary_op_f32 (ecx , which , & op) ? ; ecx . write_scalar (res , & dest) ? ; } interp_ok (()) }
};
}
