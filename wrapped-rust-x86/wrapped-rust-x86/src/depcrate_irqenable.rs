// Generated macro for enable (function)
macro_rules! Depcrate_irqenable {
() => {
// Module: crate::irq
// Provides: {"enable"}
// Dependencies: {}
# [doc = " Enable Interrupts."] # [doc = ""] # [doc = " # Safety"] # [doc = " Only allowed if we have IO privileges for the current operating level in RFlags."] pub unsafe fn enable () { asm ! ("sti") ; }
};
}
