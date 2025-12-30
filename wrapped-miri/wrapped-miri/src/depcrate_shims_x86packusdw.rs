// Generated macro for packusdw (function)
macro_rules! Depcrate_shims_x86packusdw {
() => {
// Module: crate::shims::x86
// Provides: {"packusdw"}
// Dependencies: {}
# [doc = " Converts two 32-bit integer vectors to a single 16-bit integer"] # [doc = " vector with unsigned saturation."] # [doc = ""] # [doc = " Each 128-bit chunk is treated independently (i.e., the value for"] # [doc = " the is i-th 128-bit chunk of `dest` is calculated with the i-th"] # [doc = " 128-bit chunks of `left` and `right`)."] fn packusdw < 'tcx > (ecx : & mut crate :: MiriInterpCx < 'tcx > , left : & OpTy < 'tcx > , right : & OpTy < 'tcx > , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , () > { pack_generic (ecx , left , right , dest , | op | { let op = op . to_i32 () ? ; let res = u16 :: try_from (op) . unwrap_or (if op < 0 { 0 } else { u16 :: MAX }) ; interp_ok (Scalar :: from_u16 (res)) }) }
};
}
