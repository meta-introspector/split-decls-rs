// Generated macro for enc_ccmp_imm (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_ccmp_imm {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_ccmp_imm"}
// Dependencies: {}
fn enc_ccmp_imm (size : OperandSize , rn : Reg , imm : UImm5 , nzcv : NZCV , cond : Cond) -> u32 { 0b0_1_1_11010010_00000_0000_10_00000_0_0000 | size . sf_bit () << 31 | imm . bits () << 16 | cond . bits () << 12 | machreg_to_gpr (rn) << 5 | nzcv . bits () }
};
}
