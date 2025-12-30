// Generated macro for impl_16 (impl)
macro_rules! Depcrate_decompressimpl_16 {
() => {
// Module: crate::decompress
// Provides: {"impl_16"}
// Dependencies: {}
impl io :: Read for DecompressionReader { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { match self . rdr { Ok (ref mut rdr) => rdr . read (buf) , Err (ref mut rdr) => rdr . read (buf) , } } }
};
}
