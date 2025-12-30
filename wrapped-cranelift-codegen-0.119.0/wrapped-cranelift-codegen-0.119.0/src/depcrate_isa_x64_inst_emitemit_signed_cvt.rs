// Generated macro for emit_signed_cvt (function)
macro_rules! Depcrate_isa_x64_inst_emitemit_signed_cvt {
() => {
// Module: crate::isa::x64::inst::emit
// Provides: {"emit_signed_cvt"}
// Dependencies: {}
# [doc = " A small helper to generate a signed conversion instruction."] fn emit_signed_cvt (sink : & mut MachBuffer < Inst > , info : & EmitInfo , state : & mut EmitState , src : Reg , dst : Writable < Reg > , to_f64 : bool ,) { let op = if to_f64 { SseOpcode :: Cvtsi2sd } else { SseOpcode :: Cvtsi2ss } ; let dst = WritableXmm :: from_writable_reg (dst) . unwrap () ; Inst :: CvtIntToFloat { op , dst , src1 : dst . to_reg () , src2 : GprMem :: unwrap_new (RegMem :: reg (src)) , src2_size : OperandSize :: Size64 , } . emit (sink , info , state) ; }
};
}
