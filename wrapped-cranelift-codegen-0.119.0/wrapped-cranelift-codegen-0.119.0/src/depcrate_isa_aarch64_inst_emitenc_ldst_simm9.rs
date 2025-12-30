// Generated macro for enc_ldst_simm9 (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_ldst_simm9 {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_ldst_simm9"}
// Dependencies: {}
fn enc_ldst_simm9 (op_31_22 : u32 , simm9 : SImm9 , op_11_10 : u32 , rn : Reg , rd : Reg) -> u32 { (op_31_22 << 22) | (simm9 . bits () << 12) | (op_11_10 << 10) | (machreg_to_gpr (rn) << 5) | machreg_to_gpr_or_vec (rd) }
};
}
