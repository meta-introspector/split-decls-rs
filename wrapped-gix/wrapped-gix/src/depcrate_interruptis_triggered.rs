// Generated macro for is_triggered (function)
macro_rules! Depcrate_interruptis_triggered {
() => {
// Module: crate::interrupt
// Provides: {"is_triggered"}
// Dependencies: {}
# [doc = " Returns true if an interrupt is requested."] pub fn is_triggered () -> bool { IS_INTERRUPTED . load (Ordering :: Relaxed) }
};
}
