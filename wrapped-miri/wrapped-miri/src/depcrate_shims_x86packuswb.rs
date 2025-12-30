// Generated macro for packuswb (function)
macro_rules! Depcrate_shims_x86packuswb {
() => {
// Module: crate::shims::x86
// Provides: {"packuswb"}
// Dependencies: {}
# [doc = " Converts two 16-bit signed integer vectors to a single 8-bit"] # [doc = " unsigned integer vector with saturation."] # [doc = ""] # [doc = " Each 128-bit chunk is treated independently (i.e., the value for"] # [doc = " the is i-th 128-bit chunk of `dest` is calculated with the i-th"] # [doc = " 128-bit chunks of `left` and `right`)."] fn packuswb < 'tcx > (ecx : & mut crate :: MiriInterpCx < 'tcx > , left : & OpTy < 'tcx > , right : & OpTy < 'tcx > , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , () > { pack_generic (ecx , left , right , dest , | op | { let op = op . to_i16 () ? ; let res = u8 :: try_from (op) . unwrap_or (if op < 0 { 0 } else { u8 :: MAX }) ; interp_ok (Scalar :: from_u8 (res)) }) }
};
}
