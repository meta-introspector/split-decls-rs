// Generated macro for impl_722 (impl)
macro_rules! Depcrate_open_optionsimpl_722 {
() => {
// Module: crate::open::options
// Provides: {"impl_722"}
// Dependencies: {}
# [doc = " Generic modification"] impl Options { # [doc = " An adapter to allow calling any builder method on this instance despite only having a mutable reference."] pub fn modify (& mut self , f : impl FnOnce (Self) -> Self) { * self = f (std :: mem :: take (self)) ; } }
};
}
