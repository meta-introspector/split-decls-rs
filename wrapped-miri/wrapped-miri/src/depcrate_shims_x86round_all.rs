// Generated macro for round_all (function)
macro_rules! Depcrate_shims_x86round_all {
() => {
// Module: crate::shims::x86
// Provides: {"round_all"}
// Dependencies: {}
fn round_all < 'tcx , F : rustc_apfloat :: Float > (ecx : & mut crate :: MiriInterpCx < 'tcx > , op : & OpTy < 'tcx > , rounding : & OpTy < 'tcx > , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , () > { let (op , op_len) = ecx . project_to_simd (op) ? ; let (dest , dest_len) = ecx . project_to_simd (dest) ? ; assert_eq ! (dest_len , op_len) ; let rounding = rounding_from_imm (ecx . read_scalar (rounding) ? . to_i32 () ?) ? ; for i in 0 .. dest_len { let op : F = ecx . read_scalar (& ecx . project_index (& op , i) ?) ? . to_float () ? ; let res = op . round_to_integral (rounding) . value ; ecx . write_scalar (Scalar :: from_uint (res . to_bits () , Size :: from_bits (F :: BITS)) , & ecx . project_index (& dest , i) ? ,) ? ; } interp_ok (()) }
};
}
