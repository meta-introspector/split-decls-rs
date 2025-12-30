// Generated macro for impl_46 (impl)
macro_rules! Depcrate_stream_binary_readerimpl_46 {
() => {
// Module: crate::stream::binary_reader
// Provides: {"impl_46"}
// Dependencies: {}
impl < R : Read > Read for PosReader < R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let count = self . reader . read (buf) ? ; self . pos . checked_add (count as u64) . expect ("file cannot be larger than `u64::MAX` bytes") ; Ok (count) } }
};
}
