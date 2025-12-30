// Generated macro for Seek (trait)
macro_rules! DepcrateSeek {
() => {
// Module: crate
// Provides: {"Seek"}
// Dependencies: {}
# [doc = " Async seek within streams."] # [doc = ""] # [doc = " This trait is the `embedded-io-async` equivalent of [`std::io::Seek`]."] pub trait Seek : ErrorType { # [doc = " Seek to an offset, in bytes, in a stream."] async fn seek (& mut self , pos : SeekFrom) -> Result < u64 , Self :: Error > ; # [doc = " Rewind to the beginning of a stream."] async fn rewind (& mut self) -> Result < () , Self :: Error > { self . seek (SeekFrom :: Start (0)) . await ? ; Ok (()) } # [doc = " Returns the current seek position from the start of the stream."] async fn stream_position (& mut self) -> Result < u64 , Self :: Error > { self . seek (SeekFrom :: Current (0)) . await } }
};
}
