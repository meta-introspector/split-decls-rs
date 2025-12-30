// Generated macro for impl_179 (impl)
macro_rules! Depcrate_gz_bufreadimpl_179 {
() => {
// Module: crate::gz::bufread
// Provides: {"impl_179"}
// Dependencies: {}
impl < R > GzEncoder < R > { # [doc = " Acquires a reference to the underlying reader."] pub fn get_ref (& self) -> & R { self . inner . get_ref () . get_ref () } # [doc = " Acquires a mutable reference to the underlying reader."] # [doc = ""] # [doc = " Note that mutation of the reader may result in surprising results if"] # [doc = " this encoder is continued to be used."] pub fn get_mut (& mut self) -> & mut R { self . inner . get_mut () . get_mut () } # [doc = " Returns the underlying stream, consuming this encoder"] pub fn into_inner (self) -> R { self . inner . into_inner () . into_inner () } }
};
}
