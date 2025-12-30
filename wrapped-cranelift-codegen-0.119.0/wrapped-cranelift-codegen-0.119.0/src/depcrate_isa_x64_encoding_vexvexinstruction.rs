// Generated macro for VexInstruction (struct)
macro_rules! Depcrate_isa_x64_encoding_vexVexInstruction {
() => {
// Module: crate::isa::x64::encoding::vex
// Provides: {"VexInstruction"}
// Dependencies: {}
# [doc = " Constructs a VEX-encoded instruction using a builder pattern. This approach makes it visually"] # [doc = " easier to transform something the manual's syntax, `VEX.128.66.0F 73 /7 ib` to code:"] # [doc = " `VexInstruction::new().length(...).prefix(...).map(...).w(true).opcode(0x1F).reg(...).rm(...)`."] pub struct VexInstruction { length : VexVectorLength , prefix : LegacyPrefixes , map : OpcodeMap , opcode : u8 , w : bool , reg : u8 , rm : RegisterOrAmode , vvvv : Option < Register > , imm : Option < u8 > , }
};
}
