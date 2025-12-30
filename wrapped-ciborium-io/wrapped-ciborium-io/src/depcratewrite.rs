// Generated macro for Write (trait)
macro_rules! DepcrateWrite {
() => {
// Module: crate
// Provides: {"Write"}
// Dependencies: {}
# [doc = " A trait indicating a type that can write bytes"] # [doc = ""] # [doc = " Note that this is similar to `std::io::Write`, but simplified for use in a"] # [doc = " `no_std` context."] pub trait Write { # [doc = " The error type"] type Error ; # [doc = " Writes all bytes from `data` or fails"] fn write_all (& mut self , data : & [u8]) -> Result < () , Self :: Error > ; # [doc = " Flushes all output"] fn flush (& mut self) -> Result < () , Self :: Error > ; }
};
}
