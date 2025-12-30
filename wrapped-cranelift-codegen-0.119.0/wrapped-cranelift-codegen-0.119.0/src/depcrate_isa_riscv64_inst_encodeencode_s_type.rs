// Generated macro for encode_s_type (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_s_type {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_s_type"}
// Dependencies: {}
# [doc = " Encode an S-type instruction."] # [doc = ""] # [doc = " Layout:"] # [doc = " 0-------6-7-------11-12------14-15------19-20---24-25-------------31"] # [doc = " | Opcode | imm[4:0] |  width   |   base   |  src  |    imm[11:5]   |"] pub fn encode_s_type (opcode : u32 , width : u32 , base : Reg , src : Reg , offset : Imm12) -> u32 { let mut bits = 0 ; bits |= unsigned_field_width (opcode , 7) ; bits |= (offset . bits () & 0b11111) << 7 ; bits |= unsigned_field_width (width , 3) << 12 ; bits |= reg_to_gpr_num (base) << 15 ; bits |= reg_to_gpr_num (src) << 20 ; bits |= unsigned_field_width (offset . bits () >> 5 , 7) << 25 ; bits }
};
}
