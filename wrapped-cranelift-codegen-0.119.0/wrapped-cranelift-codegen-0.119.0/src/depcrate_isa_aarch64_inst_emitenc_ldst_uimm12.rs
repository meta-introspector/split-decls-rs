// Generated macro for enc_ldst_uimm12 (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_ldst_uimm12 {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_ldst_uimm12"}
// Dependencies: {}
fn enc_ldst_uimm12 (op_31_22 : u32 , uimm12 : UImm12Scaled , rn : Reg , rd : Reg) -> u32 { (op_31_22 << 22) | (0b1 << 24) | (uimm12 . bits () << 10) | (machreg_to_gpr (rn) << 5) | machreg_to_gpr_or_vec (rd) }
};
}
