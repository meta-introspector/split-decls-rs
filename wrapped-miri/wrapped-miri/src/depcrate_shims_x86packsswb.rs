// Generated macro for packsswb (function)
macro_rules! Depcrate_shims_x86packsswb {
() => {
// Module: crate::shims::x86
// Provides: {"packsswb"}
// Dependencies: {}
# [doc = " Converts two 16-bit integer vectors to a single 8-bit integer"] # [doc = " vector with signed saturation."] # [doc = ""] # [doc = " Each 128-bit chunk is treated independently (i.e., the value for"] # [doc = " the is i-th 128-bit chunk of `dest` is calculated with the i-th"] # [doc = " 128-bit chunks of `left` and `right`)."] fn packsswb < 'tcx > (ecx : & mut crate :: MiriInterpCx < 'tcx > , left : & OpTy < 'tcx > , right : & OpTy < 'tcx > , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , () > { pack_generic (ecx , left , right , dest , | op | { let op = op . to_i16 () ? ; let res = i8 :: try_from (op) . unwrap_or (if op < 0 { i8 :: MIN } else { i8 :: MAX }) ; interp_ok (Scalar :: from_i8 (res)) }) }
};
}
