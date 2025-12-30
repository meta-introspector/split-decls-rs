// Generated macro for WritebackOnDrop (struct)
macro_rules! Depcrate___macros_writebackWritebackOnDrop {
() => {
// Module: crate::__macros::writeback
// Provides: {"WritebackOnDrop"}
// Dependencies: {}
# [derive (Debug)] pub struct WritebackOnDrop < T : Message > { # [doc = " A copy of the argument, so that we can retain it after the message"] # [doc = " send."] # [doc = ""] # [doc = " Ideally, we'd work with e.g. `&mut *mut T`, but we can't do that"] # [doc = " inside the generic context of `MessageArguments::__invoke`, while"] # [doc = " working within Rust's aliasing rules."] ptr : NonNull < * mut T > , # [doc = " The old value, stored so that we can release if after the message"] # [doc = " send."] old : NonNull < T > , }
};
}
