// Generated macro for impl_203 (impl)
macro_rules! Depcrate_gz_readimpl_203 {
() => {
// Module: crate::gz::read
// Provides: {"impl_203"}
// Dependencies: {}
impl < R : Read > GzEncoder < R > { # [doc = " Creates a new encoder which will use the given compression level."] # [doc = ""] # [doc = " The encoder is not configured specially for the emitted header. For"] # [doc = " header configuration, see the `GzBuilder` type."] # [doc = ""] # [doc = " The data read from the stream `r` will be compressed and available"] # [doc = " through the returned reader."] pub fn new (r : R , level : Compression) -> GzEncoder < R > { GzBuilder :: new () . read (r , level) } }
};
}
