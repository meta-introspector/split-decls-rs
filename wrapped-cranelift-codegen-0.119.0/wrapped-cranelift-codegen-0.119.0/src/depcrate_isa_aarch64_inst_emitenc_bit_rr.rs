// Generated macro for enc_bit_rr (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_bit_rr {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_bit_rr"}
// Dependencies: {}
fn enc_bit_rr (size : u32 , opcode2 : u32 , opcode1 : u32 , rn : Reg , rd : Writable < Reg >) -> u32 { (0b01011010110 << 21) | size << 31 | opcode2 << 16 | opcode1 << 10 | machreg_to_gpr (rn) << 5 | machreg_to_gpr (rd . to_reg ()) }
};
}
