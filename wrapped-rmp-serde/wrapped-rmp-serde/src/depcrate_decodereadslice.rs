// Generated macro for ReadSlice (trait)
macro_rules! Depcrate_decodeReadSlice {
() => {
// Module: crate::decode
// Provides: {"ReadSlice"}
// Dependencies: {}
# [doc = " Extends the `Read` trait by allowing to read slices directly by borrowing bytes."] # [doc = ""] # [doc = " Used to allow zero-copy reading."] pub trait ReadSlice < 'de > : Read { # [doc = " Reads the exact number of bytes from the underlying byte-array."] fn read_slice < 'a > (& 'a mut self , len : usize) -> Result < Reference < 'de , 'a , [u8] > , io :: Error > ; }
};
}
