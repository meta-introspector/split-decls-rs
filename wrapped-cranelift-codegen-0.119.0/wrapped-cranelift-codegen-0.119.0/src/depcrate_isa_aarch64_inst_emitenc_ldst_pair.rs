// Generated macro for enc_ldst_pair (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_ldst_pair {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_ldst_pair"}
// Dependencies: {}
fn enc_ldst_pair (op_31_22 : u32 , simm7 : SImm7Scaled , rn : Reg , rt : Reg , rt2 : Reg) -> u32 { (op_31_22 << 22) | (simm7 . bits () << 15) | (machreg_to_gpr (rt2) << 10) | (machreg_to_gpr (rn) << 5) | machreg_to_gpr (rt) }
};
}
