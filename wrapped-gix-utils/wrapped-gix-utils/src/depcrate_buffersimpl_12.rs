// Generated macro for impl_12 (impl)
macro_rules! Depcrate_buffersimpl_12 {
() => {
// Module: crate::buffers
// Provides: {"impl_12"}
// Dependencies: {}
impl Buffers { # [doc = " Clear all buffers, which should be called once processing is done."] pub fn clear (& mut self) { self . src . clear () ; self . dest . clear () ; } # [doc = " Must be called after every change (i.e. when it's known that `dest` was written."] pub fn swap (& mut self) { std :: mem :: swap (& mut self . src , & mut self . dest) ; } }
};
}
