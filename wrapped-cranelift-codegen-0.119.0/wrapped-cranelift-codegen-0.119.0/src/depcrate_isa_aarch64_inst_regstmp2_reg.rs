// Generated macro for tmp2_reg (function)
macro_rules! Depcrate_isa_aarch64_inst_regstmp2_reg {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"tmp2_reg"}
// Dependencies: {}
# [doc = " Get a reference to the second temp register. We need this in some edge cases"] # [doc = " where we need both the spilltmp and another temporary."] # [doc = ""] # [doc = " We use x17 (aka IP1), the other \"interprocedural\"/linker-veneer scratch reg that is"] # [doc = " free to use otherwise."] pub fn tmp2_reg () -> Reg { xreg (17) }
};
}
