// Generated macro for a64_reg_index (function)
macro_rules! Depcrate_asma64_reg_index {
() => {
// Module: crate::asm
// Provides: {"a64_reg_index"}
// Dependencies: {}
# [doc = " If the register is an AArch64 integer register then return its index."] fn a64_reg_index (reg : InlineAsmReg) -> Option < u32 > { match reg { InlineAsmReg :: AArch64 (r) => r . reg_index () , _ => None , } }
};
}
