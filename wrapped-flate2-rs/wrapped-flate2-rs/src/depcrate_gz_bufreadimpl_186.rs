// Generated macro for impl_186 (impl)
macro_rules! Depcrate_gz_bufreadimpl_186 {
() => {
// Module: crate::gz::bufread
// Provides: {"impl_186"}
// Dependencies: {}
impl < R > GzDecoder < R > { # [doc = " Returns the header associated with this stream, if it was valid"] pub fn header (& self) -> Option < & GzHeader > { match & self . state { GzState :: Body (header) | GzState :: Finished (header , _ , _) => Some (header) , GzState :: End (header) => header . as_ref () , _ => None , } } # [doc = " Acquires a reference to the underlying reader."] pub fn get_ref (& self) -> & R { self . reader . get_ref () . get_ref () } # [doc = " Acquires a mutable reference to the underlying stream."] # [doc = ""] # [doc = " Note that mutation of the stream may result in surprising results if"] # [doc = " this decoder is continued to be used."] pub fn get_mut (& mut self) -> & mut R { self . reader . get_mut () . get_mut () } # [doc = " Consumes this decoder, returning the underlying reader."] pub fn into_inner (self) -> R { self . reader . into_inner () . into_inner () } }
};
}
