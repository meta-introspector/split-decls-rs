// Generated macro for RegMemImm (enum)
macro_rules! Depcrate_isa_x64_inst_argsRegMemImm {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"RegMemImm"}
// Dependencies: {}
# [doc = " An operand which is either an integer Register, a value in Memory or an Immediate.  This can"] # [doc = " denote an 8, 16, 32 or 64 bit value.  For the Immediate form, in the 8- and 16-bit case, only"] # [doc = " the lower 8 or 16 bits of `simm32` is relevant.  In the 64-bit case, the value denoted by"] # [doc = " `simm32` is its sign-extension out to 64 bits."] # [derive (Clone , Debug)] pub enum RegMemImm { # [doc = " A register operand."] Reg { # [doc = " The underlying register."] reg : Reg , } , # [doc = " A memory operand."] Mem { # [doc = " The memory address."] addr : SyntheticAmode , } , # [doc = " An immediate operand."] Imm { # [doc = " The immediate value."] simm32 : u32 , } , }
};
}
