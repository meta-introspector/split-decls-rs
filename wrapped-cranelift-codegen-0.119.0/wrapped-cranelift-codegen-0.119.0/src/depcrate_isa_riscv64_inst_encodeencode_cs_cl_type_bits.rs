// Generated macro for encode_cs_cl_type_bits (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_cs_cl_type_bits {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_cs_cl_type_bits"}
// Dependencies: {}
fn encode_cs_cl_type_bits (op : COpcodeSpace , funct3 : u32 , size : u32 , dest_src : Reg , base : Reg , imm : Uimm5 ,) -> u16 { let imm = imm . bits () ; let imm2 = match size { 4 => ((imm >> 4) & 1) | ((imm & 1) << 1) , 8 => (imm >> 3) & 0b11 , _ => unreachable ! () , } ; let imm3 = match size { 4 => (imm >> 1) & 0b111 , 8 => (imm >> 0) & 0b111 , _ => unreachable ! () , } ; let mut bits = 0 ; bits |= unsigned_field_width (op . bits () , 2) ; bits |= reg_to_compressed_gpr_num (dest_src) << 2 ; bits |= unsigned_field_width (imm2 as u32 , 2) << 5 ; bits |= reg_to_compressed_gpr_num (base) << 7 ; bits |= unsigned_field_width (imm3 as u32 , 3) << 10 ; bits |= unsigned_field_width (funct3 , 3) << 13 ; bits . try_into () . unwrap () }
};
}
