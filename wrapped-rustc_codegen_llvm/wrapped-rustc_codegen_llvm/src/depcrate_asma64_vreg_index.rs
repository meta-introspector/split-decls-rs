// Generated macro for a64_vreg_index (function)
macro_rules! Depcrate_asma64_vreg_index {
() => {
// Module: crate::asm
// Provides: {"a64_vreg_index"}
// Dependencies: {}
# [doc = " If the register is an AArch64 vector register then return its index."] fn a64_vreg_index (reg : InlineAsmReg) -> Option < u32 > { match reg { InlineAsmReg :: AArch64 (reg) => reg . vreg_index () , _ => None , } }
};
}
