// Generated macro for encode_r_type_bits (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_r_type_bits {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_r_type_bits"}
// Dependencies: {}
# [doc = " Layout:"] # [doc = " 0-------6-7-------11-12------14-15------19-20------24-25-------31"] # [doc = " | Opcode |   rd     |  funct3  |   rs1    |   rs2    |   funct7  |"] fn encode_r_type_bits (opcode : u32 , rd : u32 , funct3 : u32 , rs1 : u32 , rs2 : u32 , funct7 : u32) -> u32 { let mut bits = 0 ; bits |= unsigned_field_width (opcode , 7) ; bits |= unsigned_field_width (rd , 5) << 7 ; bits |= unsigned_field_width (funct3 , 3) << 12 ; bits |= unsigned_field_width (rs1 , 5) << 15 ; bits |= unsigned_field_width (rs2 , 5) << 20 ; bits |= unsigned_field_width (funct7 , 7) << 25 ; bits }
};
}
