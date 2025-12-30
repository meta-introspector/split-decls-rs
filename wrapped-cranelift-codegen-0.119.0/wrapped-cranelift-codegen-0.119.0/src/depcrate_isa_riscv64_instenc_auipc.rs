// Generated macro for enc_auipc (function)
macro_rules! Depcrate_isa_riscv64_instenc_auipc {
() => {
// Module: crate::isa::riscv64::inst
// Provides: {"enc_auipc"}
// Dependencies: {}
pub (crate) fn enc_auipc (rd : Writable < Reg > , imm : Imm20) -> u32 { let x = 0b0010111 | reg_to_gpr_num (rd . to_reg ()) << 7 | imm . bits () << 12 ; x }
};
}
