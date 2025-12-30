// Generated macro for encode_i_type_bits (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_i_type_bits {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_i_type_bits"}
// Dependencies: {}
# [doc = " Layout:"] # [doc = " 0-------6-7-------11-12------14-15------19-20------------------31"] # [doc = " | Opcode |   rd     |  width   |   rs1    |     Offset[11:0]    |"] fn encode_i_type_bits (opcode : u32 , rd : u32 , funct3 : u32 , rs1 : u32 , offset : u32) -> u32 { let mut bits = 0 ; bits |= unsigned_field_width (opcode , 7) ; bits |= unsigned_field_width (rd , 5) << 7 ; bits |= unsigned_field_width (funct3 , 3) << 12 ; bits |= unsigned_field_width (rs1 , 5) << 15 ; bits |= unsigned_field_width (offset , 12) << 20 ; bits }
};
}
