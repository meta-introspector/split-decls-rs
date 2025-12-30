// Generated macro for unary_op_ss (function)
macro_rules! Depcrate_shims_x86unary_op_ss {
() => {
// Module: crate::shims::x86
// Provides: {"unary_op_ss"}
// Dependencies: {}
# [doc = " Performs `which` operation on the first component of `op` and copies"] # [doc = " the other components. The result is stored in `dest`."] fn unary_op_ss < 'tcx > (ecx : & mut crate :: MiriInterpCx < 'tcx > , which : FloatUnaryOp , op : & OpTy < 'tcx > , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , () > { let (op , op_len) = ecx . project_to_simd (op) ? ; let (dest , dest_len) = ecx . project_to_simd (dest) ? ; assert_eq ! (dest_len , op_len) ; let res0 = unary_op_f32 (ecx , which , & ecx . read_immediate (& ecx . project_index (& op , 0) ?) ?) ? ; ecx . write_scalar (res0 , & ecx . project_index (& dest , 0) ?) ? ; for i in 1 .. dest_len { ecx . copy_op (& ecx . project_index (& op , i) ? , & ecx . project_index (& dest , i) ?) ? ; } interp_ok (()) }
};
}
