// Generated macro for pinned_reg (function)
macro_rules! Depcrate_isa_x64_inst_regspinned_reg {
() => {
// Module: crate::isa::x64::inst::regs
// Provides: {"pinned_reg"}
// Dependencies: {}
# [doc = " The pinned register on this architecture."] # [doc = " It must be the same as Spidermonkey's HeapReg, as found in this file."] # [doc = " https://searchfox.org/mozilla-central/source/js/src/jit/x64/Assembler-x64.h#99"] pub (crate) fn pinned_reg () -> Reg { r15 () }
};
}
