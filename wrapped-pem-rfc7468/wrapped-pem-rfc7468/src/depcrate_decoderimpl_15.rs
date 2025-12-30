// Generated macro for impl_15 (impl)
macro_rules! Depcrate_decoderimpl_15 {
() => {
// Module: crate::decoder
// Provides: {"impl_15"}
// Dependencies: {}
# [cfg (feature = "std")] impl io :: Read for Decoder < '_ > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . base64 . read (buf) } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { self . base64 . read_to_end (buf) } fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { self . base64 . read_exact (buf) } }
};
}
