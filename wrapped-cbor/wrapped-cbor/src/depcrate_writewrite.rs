// Generated macro for Write (trait)
macro_rules! Depcrate_writeWrite {
() => {
// Module: crate::write
// Provides: {"Write"}
// Dependencies: {}
# [cfg (feature = "unsealed_read_write")] # [doc = " A sink for serialized CBOR."] # [doc = ""] # [doc = " This trait is similar to the [`Write`]() trait in the standard library,"] # [doc = " but has a smaller and more general API."] # [doc = ""] # [doc = " Any object implementing `std::io::Write`"] # [doc = " can be wrapped in an [`IoWrite`](../write/struct.IoWrite.html) that implements"] # [doc = " this trait for the underlying object."] # [doc = ""] # [doc = " This trait is sealed by default, enabling the `unsealed_read_write` feature removes this bound"] # [doc = " to allow objects outside of this crate to implement this trait."] pub trait Write { # [doc = " The type of error returned when a write operation fails."] type Error : Into < error :: Error > ; # [doc = " Attempts to write an entire buffer into this write."] fn write_all (& mut self , buf : & [u8]) -> Result < () , Self :: Error > ; }
};
}
