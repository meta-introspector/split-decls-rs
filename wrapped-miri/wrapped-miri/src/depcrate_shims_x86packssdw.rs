// Generated macro for packssdw (function)
macro_rules! Depcrate_shims_x86packssdw {
() => {
// Module: crate::shims::x86
// Provides: {"packssdw"}
// Dependencies: {}
# [doc = " Converts two 32-bit integer vectors to a single 16-bit integer"] # [doc = " vector with signed saturation."] # [doc = ""] # [doc = " Each 128-bit chunk is treated independently (i.e., the value for"] # [doc = " the is i-th 128-bit chunk of `dest` is calculated with the i-th"] # [doc = " 128-bit chunks of `left` and `right`)."] fn packssdw < 'tcx > (ecx : & mut crate :: MiriInterpCx < 'tcx > , left : & OpTy < 'tcx > , right : & OpTy < 'tcx > , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , () > { pack_generic (ecx , left , right , dest , | op | { let op = op . to_i32 () ? ; let res = i16 :: try_from (op) . unwrap_or (if op < 0 { i16 :: MIN } else { i16 :: MAX }) ; interp_ok (Scalar :: from_i16 (res)) }) }
};
}
