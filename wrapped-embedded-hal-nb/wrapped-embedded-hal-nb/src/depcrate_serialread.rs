// Generated macro for Read (trait)
macro_rules! Depcrate_serialRead {
() => {
// Module: crate::serial
// Provides: {"Read"}
// Dependencies: {}
# [doc = " Read half of a serial interface."] # [doc = ""] # [doc = " Some serial interfaces support different data sizes (8 bits, 9 bits, etc.);"] # [doc = " This can be encoded in this trait via the `Word` type parameter."] pub trait Read < Word : Copy = u8 > : ErrorType { # [doc = " Reads a single word from the serial interface"] fn read (& mut self) -> nb :: Result < Word , Self :: Error > ; }
};
}
