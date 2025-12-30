// Generated macro for unary_op_f32 (function)
macro_rules! Depcrate_shims_x86unary_op_f32 {
() => {
// Module: crate::shims::x86
// Provides: {"unary_op_f32"}
// Dependencies: {}
# [doc = " Performs `which` scalar operation on `op` and returns the result."] fn unary_op_f32 < 'tcx > (ecx : & mut crate :: MiriInterpCx < 'tcx > , which : FloatUnaryOp , op : & ImmTy < 'tcx > ,) -> InterpResult < 'tcx , Scalar > { match which { FloatUnaryOp :: Rcp => { let op = op . to_scalar () . to_f32 () ? ; let div = (Single :: from_u128 (1) . value / op) . value ; let res = math :: apply_random_float_error (ecx , div , - 12) ; interp_ok (Scalar :: from_f32 (res)) } FloatUnaryOp :: Rsqrt => { let op = op . to_scalar () . to_f32 () ? ; let rsqrt = (Single :: from_u128 (1) . value / math :: sqrt (op)) . value ; let res = math :: apply_random_float_error (ecx , rsqrt , - 12) ; interp_ok (Scalar :: from_f32 (res)) } } }
};
}
