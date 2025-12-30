// Generated macro for rounding_from_imm (function)
macro_rules! Depcrate_shims_x86rounding_from_imm {
() => {
// Module: crate::shims::x86
// Provides: {"rounding_from_imm"}
// Dependencies: {}
# [doc = " Gets equivalent `rustc_apfloat::Round` from rounding mode immediate of"] # [doc = " `round.{ss,sd,ps,pd}` intrinsics."] fn rounding_from_imm < 'tcx > (rounding : i32) -> InterpResult < 'tcx , rustc_apfloat :: Round > { match rounding & ! 0b1000 { 0b000 => interp_ok (rustc_apfloat :: Round :: NearestTiesToEven) , 0b001 => interp_ok (rustc_apfloat :: Round :: TowardNegative) , 0b010 => interp_ok (rustc_apfloat :: Round :: TowardPositive) , 0b011 => interp_ok (rustc_apfloat :: Round :: TowardZero) , 0b100 ..= 0b111 => interp_ok (rustc_apfloat :: Round :: NearestTiesToEven) , rounding => panic ! ("invalid rounding mode 0x{rounding:02x}") , } }
};
}
