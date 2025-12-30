// Generated macro for dr6_write (function)
macro_rules! Depcrate_debugregsdr6_write {
() => {
// Module: crate::debugregs
// Provides: {"dr6_write"}
// Dependencies: {}
# [doc = " Write dr6."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " Certain debug exceptions may clear bits 0-3. The remaining contents of the"] # [doc = " DR6 register are never cleared by the processor. To avoid confusion in"] # [doc = " identifying debug exceptions, debug handlers should clear the register"] # [doc = " (except bit 16, which they should set) before returning to the interrupted"] # [doc = " task)."] # [doc = ""] # [doc = " # Safety"] # [doc = " Needs CPL 0."] pub unsafe fn dr6_write (val : Dr6) { asm ! ("mov {}, %dr6" , in (reg) val . bits , options (att_syntax)) ; }
};
}
