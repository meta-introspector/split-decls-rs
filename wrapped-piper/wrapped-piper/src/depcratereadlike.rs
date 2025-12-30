// Generated macro for ReadLike (trait)
macro_rules! DepcrateReadLike {
() => {
// Module: crate
// Provides: {"ReadLike"}
// Dependencies: {}
# [doc = " A trait for reading bytes into a pipe."] trait ReadLike { # [doc = " The error type."] type Error ; # [doc = " Reads bytes into the given buffer."] fn read (& mut self , buf : & mut [u8]) -> Result < usize , Self :: Error > ; }
};
}
