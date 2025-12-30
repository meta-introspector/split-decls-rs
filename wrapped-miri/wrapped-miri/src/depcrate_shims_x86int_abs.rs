// Generated macro for int_abs (function)
macro_rules! Depcrate_shims_x86int_abs {
() => {
// Module: crate::shims::x86
// Provides: {"int_abs"}
// Dependencies: {}
# [doc = " Calculates absolute value of integers in `op` and stores the result in `dest`."] # [doc = ""] # [doc = " In case of overflow (when the operand is the minimum value), the operation"] # [doc = " will wrap around."] fn int_abs < 'tcx > (ecx : & mut crate :: MiriInterpCx < 'tcx > , op : & OpTy < 'tcx > , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , () > { let (op , op_len) = ecx . project_to_simd (op) ? ; let (dest , dest_len) = ecx . project_to_simd (dest) ? ; assert_eq ! (op_len , dest_len) ; let zero = ImmTy :: from_int (0 , op . layout . field (ecx , 0)) ; for i in 0 .. dest_len { let op = ecx . read_immediate (& ecx . project_index (& op , i) ?) ? ; let dest = ecx . project_index (& dest , i) ? ; let lt_zero = ecx . binary_op (mir :: BinOp :: Lt , & op , & zero) ? ; let res = if lt_zero . to_scalar () . to_bool () ? { ecx . unary_op (mir :: UnOp :: Neg , & op) ? } else { op } ; ecx . write_immediate (* res , & dest) ? ; } interp_ok (()) }
};
}
