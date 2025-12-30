// Generated macro for enc_jalr (function)
macro_rules! Depcrate_isa_riscv64_instenc_jalr {
() => {
// Module: crate::isa::riscv64::inst
// Provides: {"enc_jalr"}
// Dependencies: {}
pub (crate) fn enc_jalr (rd : Writable < Reg > , base : Reg , offset : Imm12) -> u32 { let x = 0b1100111 | reg_to_gpr_num (rd . to_reg ()) << 7 | 0b000 << 12 | reg_to_gpr_num (base) << 15 | offset . bits () << 20 ; x }
};
}
