// Generated macro for Read (trait)
macro_rules! DepcrateRead {
() => {
// Module: crate
// Provides: {"Read"}
// Dependencies: {}
# [doc = " A trait indicating a type that can read bytes"] # [doc = ""] # [doc = " Note that this is similar to `std::io::Read`, but simplified for use in a"] # [doc = " `no_std` context."] pub trait Read { # [doc = " The error type"] type Error ; # [doc = " Reads exactly `data.len()` bytes or fails"] fn read_exact (& mut self , data : & mut [u8]) -> Result < () , Self :: Error > ; }
};
}
