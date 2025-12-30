// Generated macro for macro_57 (macro)
macro_rules! Depcratemacro_57 {
() => {
// Module: crate
// Provides: {"macro_57"}
// Dependencies: {}
pin_project ! { # [derive (Debug)] # [project (! Unpin)] struct SendInner <'a , T > { sender : &'a Sender < T >, msg : Option < T >, listener : Option < EventListener >, # [pin] _pin : PhantomPinned } }
};
}
