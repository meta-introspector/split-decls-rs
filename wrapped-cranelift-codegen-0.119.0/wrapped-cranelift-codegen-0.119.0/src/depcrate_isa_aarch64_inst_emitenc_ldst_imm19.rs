// Generated macro for enc_ldst_imm19 (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_ldst_imm19 {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_ldst_imm19"}
// Dependencies: {}
pub (crate) fn enc_ldst_imm19 (op_31_24 : u32 , imm19 : u32 , rd : Reg) -> u32 { (op_31_24 << 24) | (imm19 << 5) | machreg_to_gpr_or_vec (rd) }
};
}
