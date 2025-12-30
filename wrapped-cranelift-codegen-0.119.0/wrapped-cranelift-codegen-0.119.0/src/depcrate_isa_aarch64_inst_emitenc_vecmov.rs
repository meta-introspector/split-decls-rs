// Generated macro for enc_vecmov (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_vecmov {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_vecmov"}
// Dependencies: {}
fn enc_vecmov (is_16b : bool , rd : Writable < Reg > , rn : Reg) -> u32 { 0b00001110_101_00000_00011_1_00000_00000 | ((is_16b as u32) << 30) | machreg_to_vec (rd . to_reg ()) | (machreg_to_vec (rn) << 16) | (machreg_to_vec (rn) << 5) }
};
}
