// Generated macro for encode_c_addi16sp (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_c_addi16sp {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_c_addi16sp"}
// Dependencies: {}
# [doc = " c.addi16sp is a regular CI op, but the immediate field is encoded in a weird way"] pub fn encode_c_addi16sp (imm : Imm6) -> u16 { let imm = imm . bits () ; let mut enc_imm = 0 ; enc_imm |= ((imm >> 5) & 1) << 5 ; enc_imm |= ((imm >> 0) & 1) << 4 ; enc_imm |= ((imm >> 2) & 1) << 3 ; enc_imm |= ((imm >> 3) & 3) << 1 ; enc_imm |= ((imm >> 1) & 1) << 0 ; let enc_imm = Imm6 :: maybe_from_i16 ((enc_imm as i16) << 10 >> 10) . unwrap () ; encode_ci_type (CiOp :: CAddi16sp , writable_stack_reg () , enc_imm) }
};
}
