// Generated macro for enc_ldst_reg (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_ldst_reg {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_ldst_reg"}
// Dependencies: {}
fn enc_ldst_reg (op_31_22 : u32 , rn : Reg , rm : Reg , s_bit : bool , extendop : Option < ExtendOp > , rd : Reg ,) -> u32 { let s_bit = if s_bit { 1 } else { 0 } ; let extend_bits = match extendop { Some (ExtendOp :: UXTW) => 0b010 , Some (ExtendOp :: SXTW) => 0b110 , Some (ExtendOp :: SXTX) => 0b111 , None => 0b011 , _ => panic ! ("bad extend mode for ld/st AMode") , } ; (op_31_22 << 22) | (1 << 21) | (machreg_to_gpr (rm) << 16) | (extend_bits << 13) | (s_bit << 12) | (0b10 << 10) | (machreg_to_gpr (rn) << 5) | machreg_to_gpr_or_vec (rd) }
};
}
