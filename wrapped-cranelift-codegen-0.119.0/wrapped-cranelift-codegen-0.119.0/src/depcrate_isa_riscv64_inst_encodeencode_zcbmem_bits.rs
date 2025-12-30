// Generated macro for encode_zcbmem_bits (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_zcbmem_bits {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_zcbmem_bits"}
// Dependencies: {}
fn encode_zcbmem_bits (op : ZcbMemOp , dest_src : Reg , base : Reg , imm : Uimm2) -> u16 { let imm = imm . bits () ; let imm = match op { ZcbMemOp :: CLh | ZcbMemOp :: CLhu | ZcbMemOp :: CSh => { debug_assert_eq ! (imm & ! 1 , 0) ; let opcode_bit = (op == ZcbMemOp :: CLh) as u8 ; imm | (opcode_bit << 1) } _ => ((imm & 1) << 1) | ((imm >> 1) & 1) , } ; let mut bits = 0 ; bits |= unsigned_field_width (op . op () . bits () , 2) ; bits |= reg_to_compressed_gpr_num (dest_src) << 2 ; bits |= unsigned_field_width (imm as u32 , 2) << 5 ; bits |= reg_to_compressed_gpr_num (base) << 7 ; bits |= unsigned_field_width (op . funct6 () , 6) << 10 ; bits . try_into () . unwrap () }
};
}
