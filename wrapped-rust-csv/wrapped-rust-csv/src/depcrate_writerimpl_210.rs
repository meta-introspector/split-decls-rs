// Generated macro for impl_210 (impl)
macro_rules! Depcrate_writerimpl_210 {
() => {
// Module: crate::writer
// Provides: {"impl_210"}
// Dependencies: {}
impl Buffer { # [doc = " Returns a slice of the buffer's current contents."] # [doc = ""] # [doc = " The slice returned may be empty."] # [inline] fn readable (& self) -> & [u8] { & self . buf [.. self . len] } # [doc = " Returns a mutable slice of the remaining space in this buffer."] # [doc = ""] # [doc = " The slice returned may be empty."] # [inline] fn writable (& mut self) -> & mut [u8] { & mut self . buf [self . len ..] } # [doc = " Indicates that `n` bytes have been written to this buffer."] # [inline] fn written (& mut self , n : usize) { self . len += n ; } # [doc = " Clear the buffer."] # [inline] fn clear (& mut self) { self . len = 0 ; } }
};
}
