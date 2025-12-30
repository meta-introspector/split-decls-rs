// Generated macro for emit_simm (function)
macro_rules! Depcrate_isa_x64_encoding_rexemit_simm {
() => {
// Module: crate::isa::x64::encoding::rex
// Provides: {"emit_simm"}
// Dependencies: {}
# [doc = " Write a suitable number of bits from an imm64 to the sink."] pub (crate) fn emit_simm < BS : ByteSink + ? Sized > (sink : & mut BS , size : u8 , simm32 : u32) { match size { 8 | 4 => sink . put4 (simm32) , 2 => sink . put2 (simm32 as u16) , 1 => sink . put1 (simm32 as u8) , _ => unreachable ! () , } }
};
}
