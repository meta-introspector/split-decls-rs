// Generated macro for reset (function)
macro_rules! Depcrate_interruptreset {
() => {
// Module: crate::interrupt
// Provides: {"reset"}
// Dependencies: {}
# [doc = " Sets the interrupt request to false, thus allowing those checking for [`is_triggered()`] to proceed."] pub fn reset () { IS_INTERRUPTED . store (false , Ordering :: SeqCst) ; }
};
}
