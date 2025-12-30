// Generated macro for Read (struct)
macro_rules! Depcrate_interruptRead {
() => {
// Module: crate::interrupt
// Provides: {"Read"}
// Dependencies: {}
# [doc = " A wrapper for implementers of [`std::io::Read`] or [`std::io::BufRead`] with interrupt support."] # [doc = ""] # [doc = " It fails a [read][std::io::Read::read] while an interrupt was requested."] pub struct Read < 'a , R > { # [doc = " The actual implementor of [`std::io::Read`] to which interrupt support will be added."] pub inner : R , # [doc = " The flag to trigger interruption"] pub should_interrupt : & 'a AtomicBool , }
};
}
