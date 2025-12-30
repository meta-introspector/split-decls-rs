// Generated macro for encode_ci_sp_load (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_ci_sp_load {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_ci_sp_load"}
// Dependencies: {}
pub fn encode_ci_sp_load (op : CiOp , rd : WritableReg , imm : Uimm6) -> u16 { let imm = imm . bits () ; let low_bits = match op { CiOp :: CLwsp => 3 , CiOp :: CLdsp | CiOp :: CFldsp => 2 , _ => unreachable ! () , } ; let high_bits = 6 - 1 - low_bits ; let mut enc_imm = 0 ; enc_imm |= imm >> (6 - high_bits) ; enc_imm |= (imm & ((1 << low_bits) - 1)) << high_bits ; enc_imm |= ((imm >> low_bits) & 1) << 5 ; let enc_imm = Imm6 :: maybe_from_i16 ((enc_imm as i16) << 10 >> 10) . unwrap () ; encode_ci_type (op , rd , enc_imm) }
};
}
