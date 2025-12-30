// Generated macro for WriteLike (trait)
macro_rules! DepcrateWriteLike {
() => {
// Module: crate
// Provides: {"WriteLike"}
// Dependencies: {}
# [doc = " A trait for writing bytes from a pipe."] trait WriteLike { # [doc = " The error type."] type Error ; # [doc = " Writes bytes from the given buffer."] fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > ; }
};
}
