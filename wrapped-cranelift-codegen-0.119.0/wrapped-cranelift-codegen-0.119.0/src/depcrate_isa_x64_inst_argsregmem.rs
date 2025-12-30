// Generated macro for RegMem (enum)
macro_rules! Depcrate_isa_x64_inst_argsRegMem {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"RegMem"}
// Dependencies: {}
# [doc = " An operand which is either an integer Register or a value in Memory.  This can denote an 8, 16,"] # [doc = " 32, 64, or 128 bit value."] # [derive (Clone , Debug)] pub enum RegMem { # [doc = " A register operand."] Reg { # [doc = " The underlying register."] reg : Reg , } , # [doc = " A memory operand."] Mem { # [doc = " The memory address."] addr : SyntheticAmode , } , }
};
}
