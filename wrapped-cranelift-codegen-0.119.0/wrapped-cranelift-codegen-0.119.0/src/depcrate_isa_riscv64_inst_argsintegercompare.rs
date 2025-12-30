// Generated macro for IntegerCompare (struct)
macro_rules! Depcrate_isa_riscv64_inst_argsIntegerCompare {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"IntegerCompare"}
// Dependencies: {}
# [doc = " risc-v always take two register to compare"] # [derive (Clone , Copy , Debug)] pub struct IntegerCompare { pub (crate) kind : IntCC , pub (crate) rs1 : Reg , pub (crate) rs2 : Reg , }
};
}
