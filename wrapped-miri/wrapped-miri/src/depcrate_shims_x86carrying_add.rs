// Generated macro for carrying_add (function)
macro_rules! Depcrate_shims_x86carrying_add {
() => {
// Module: crate::shims::x86
// Provides: {"carrying_add"}
// Dependencies: {}
# [doc = " Calcultates either `a + b + cb_in` or `a - b - cb_in` depending on the value"] # [doc = " of `op` and returns both the sum and the overflow bit. `op` is expected to be"] # [doc = " either one of `mir::BinOp::AddWithOverflow` and `mir::BinOp::SubWithOverflow`."] fn carrying_add < 'tcx > (ecx : & mut crate :: MiriInterpCx < 'tcx > , cb_in : & OpTy < 'tcx > , a : & OpTy < 'tcx > , b : & OpTy < 'tcx > , op : mir :: BinOp ,) -> InterpResult < 'tcx , (ImmTy < 'tcx > , Scalar) > { assert ! (op == mir :: BinOp :: AddWithOverflow || op == mir :: BinOp :: SubWithOverflow) ; let cb_in = ecx . read_scalar (cb_in) ? . to_u8 () ? != 0 ; let a = ecx . read_immediate (a) ? ; let b = ecx . read_immediate (b) ? ; let (sum , overflow1) = ecx . binary_op (op , & a , & b) ? . to_pair (ecx) ; let (sum , overflow2) = ecx . binary_op (op , & sum , & ImmTy :: from_uint (cb_in , a . layout)) ? . to_pair (ecx) ; let cb_out = overflow1 . to_scalar () . to_bool () ? | overflow2 . to_scalar () . to_bool () ? ; interp_ok ((sum , Scalar :: from_u8 (cb_out . into ()))) }
};
}
