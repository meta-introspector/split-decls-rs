// Generated macro for enc_ccmp (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_ccmp {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_ccmp"}
// Dependencies: {}
fn enc_ccmp (size : OperandSize , rn : Reg , rm : Reg , nzcv : NZCV , cond : Cond) -> u32 { 0b0_1_1_11010010_00000_0000_00_00000_0_0000 | size . sf_bit () << 31 | machreg_to_gpr (rm) << 16 | cond . bits () << 12 | machreg_to_gpr (rn) << 5 | nzcv . bits () }
};
}
