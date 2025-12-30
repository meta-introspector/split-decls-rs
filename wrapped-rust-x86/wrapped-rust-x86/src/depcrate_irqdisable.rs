// Generated macro for disable (function)
macro_rules! Depcrate_irqdisable {
() => {
// Module: crate::irq
// Provides: {"disable"}
// Dependencies: {}
# [doc = " Disable Interrupts."] # [doc = ""] # [doc = " # Safety"] # [doc = " Only allowed if we have IO privileges for the current operating level in RFlags."] pub unsafe fn disable () { asm ! ("cli") ; }
};
}
