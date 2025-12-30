// Generated macro for Write (struct)
macro_rules! Depcrate_interruptWrite {
() => {
// Module: crate::interrupt
// Provides: {"Write"}
// Dependencies: {}
# [doc = " A wrapper for implementers of [`std::io::Write`] with interrupt checks on each write call."] # [doc = ""] # [doc = " It fails a [write][std::io::Write::write] while an interrupt was requested."] pub struct Write < 'a , W > { # [doc = " The actual implementor of [`std::io::Write`] to which interrupt support will be added."] pub inner : W , # [doc = " The flag to trigger interruption"] pub should_interrupt : & 'a AtomicBool , }
};
}
