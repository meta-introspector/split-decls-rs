// Generated macro for impl_1815 (impl)
macro_rules! Depcrate_shims_x86impl_1815 {
() => {
// Module: crate::shims::x86
// Provides: {"impl_1815"}
// Dependencies: {}
impl FloatBinOp { # [doc = " Convert from the `imm` argument used to specify the comparison"] # [doc = " operation in intrinsics such as `llvm.x86.sse.cmp.ss`."] fn cmp_from_imm < 'tcx > (ecx : & crate :: MiriInterpCx < 'tcx > , imm : i8 , intrinsic : Symbol ,) -> InterpResult < 'tcx , Self > { if imm & ! 0b1_1111 != 0 { panic ! ("invalid `imm` parameter of {intrinsic}: 0x{imm:x}") ; } let (gt , lt , eq , mut unord) = match imm & 0b111 { 0x0 => (false , false , true , false) , 0x1 => (false , true , false , false) , 0x2 => (false , true , true , false) , 0x3 => (false , false , false , true) , 0x4 => (true , true , false , true) , 0x5 => (true , false , true , true) , 0x6 => (true , false , false , true) , 0x7 => (true , true , true , false) , _ => unreachable ! () , } ; if imm & 0b1000 != 0 { ecx . expect_target_feature_for_intrinsic (intrinsic , "avx") ? ; unord = ! unord ; } interp_ok (Self :: Cmp { gt , lt , eq , unord }) } }
};
}
