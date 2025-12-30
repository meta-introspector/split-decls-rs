// Generated macro for macro_63 (macro)
macro_rules! Depcratemacro_63 {
() => {
// Module: crate
// Provides: {"macro_63"}
// Dependencies: {}
pin_project ! { # [derive (Debug)] # [project (! Unpin)] struct ClosedInner <'a , T > { sender : &'a Sender < T >, listener : Option < EventListener >, # [pin] _pin : PhantomPinned } }
};
}
