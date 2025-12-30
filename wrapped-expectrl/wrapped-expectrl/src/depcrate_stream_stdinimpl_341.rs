// Generated macro for impl_341 (impl)
macro_rules! Depcrate_stream_stdinimpl_341 {
() => {
// Module: crate::stream::stdin
// Provides: {"impl_341"}
// Dependencies: {}
# [cfg (not (feature = "async"))] impl Read for Stdin { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . read (buf) } }
};
}
