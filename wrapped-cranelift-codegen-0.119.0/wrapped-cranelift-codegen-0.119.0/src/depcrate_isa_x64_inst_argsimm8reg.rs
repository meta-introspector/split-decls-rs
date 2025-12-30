// Generated macro for Imm8Reg (enum)
macro_rules! Depcrate_isa_x64_inst_argsImm8Reg {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"Imm8Reg"}
// Dependencies: {}
# [doc = " An operand which is either an 8-bit integer immediate or a register."] # [derive (Clone , Debug)] pub enum Imm8Reg { # [doc = " 8-bit immediate operand."] Imm8 { # [doc = " The 8-bit immediate value."] imm : u8 , } , # [doc = " A register operand."] Reg { # [doc = " The underlying register."] reg : Reg , } , }
};
}
