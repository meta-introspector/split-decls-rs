// Generated macro for EvexInstruction (struct)
macro_rules! Depcrate_isa_x64_encoding_evexEvexInstruction {
() => {
// Module: crate::isa::x64::encoding::evex
// Provides: {"EvexInstruction"}
// Dependencies: {}
# [doc = " Constructs an EVEX-encoded instruction using a builder pattern. This approach makes it visually"] # [doc = " easier to transform something the manual's syntax, `EVEX.256.66.0F38.W1 1F /r` to code:"] # [doc = " `EvexInstruction::new().length(...).prefix(...).map(...).w(true).opcode(0x1F).reg(...).rm(...)`."] pub struct EvexInstruction { bits : u32 , opcode : u8 , reg : Register , rm : RegisterOrAmode , tuple_type : Option < Avx512TupleType > , imm : Option < u8 > , }
};
}
