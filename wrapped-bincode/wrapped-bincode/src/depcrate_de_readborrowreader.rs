// Generated macro for BorrowReader (trait)
macro_rules! Depcrate_de_readBorrowReader {
() => {
// Module: crate::de::read
// Provides: {"BorrowReader"}
// Dependencies: {}
# [doc = " A reader for borrowed data. Implementers of this must also implement the [Reader] trait. See the module documentation for more information."] pub trait BorrowReader < 'storage > : Reader { # [doc = " Read exactly `length` bytes and return a slice to this data. If not enough bytes could be read, an error should be returned."] # [doc = ""] # [doc = " *note*: Exactly `length` bytes must be returned. If less bytes are returned, bincode may panic. If more bytes are returned, the excess bytes may be discarded."] fn take_bytes (& mut self , length : usize) -> Result < & 'storage [u8] , DecodeError > ; }
};
}
