// Generated macro for UnderlyingWrite (trait)
macro_rules! Depcrate_encodeUnderlyingWrite {
() => {
// Module: crate::encode
// Provides: {"UnderlyingWrite"}
// Dependencies: {}
# [doc = " Obtain the underlying writer."] pub trait UnderlyingWrite { # [doc = " Underlying writer type."] type Write : Write ; # [doc = " Gets a reference to the underlying writer."] fn get_ref (& self) -> & Self :: Write ; # [doc = " Gets a mutable reference to the underlying writer."] # [doc = ""] # [doc = " It is inadvisable to directly write to the underlying writer."] fn get_mut (& mut self) -> & mut Self :: Write ; # [doc = " Unwraps this `Serializer`, returning the underlying writer."] fn into_inner (self) -> Self :: Write ; }
};
}
