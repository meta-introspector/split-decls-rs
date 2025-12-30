// Generated macro for Buffer (trait)
macro_rules! DepcrateBuffer {
() => {
// Module: crate
// Provides: {"Buffer"}
// Dependencies: {}
# [doc = " In-place encryption/decryption byte buffers."] # [doc = ""] # [doc = " This trait defines the set of methods needed to support in-place operations"] # [doc = " on a `Vec`-like data type."] pub trait Buffer : AsRef < [u8] > + AsMut < [u8] > { # [doc = " Get the length of the buffer"] fn len (& self) -> usize { self . as_ref () . len () } # [doc = " Is the buffer empty?"] fn is_empty (& self) -> bool { self . as_ref () . is_empty () } # [doc = " Extend this buffer from the given slice"] fn extend_from_slice (& mut self , other : & [u8]) -> Result < () > ; # [doc = " Truncate this buffer to the given size"] fn truncate (& mut self , len : usize) ; }
};
}
