// Generated macro for impl_2122 (impl)
macro_rules! Depcrate_io_allow_stdimpl_2122 {
() => {
// Module: crate::io::allow_std
// Provides: {"impl_2122"}
// Dependencies: {}
impl < T > AllowStdIo < T > { # [doc = " Creates a new `AllowStdIo` from an existing IO object."] pub fn new (io : T) -> Self { Self (io) } # [doc = " Returns a reference to the contained IO object."] pub fn get_ref (& self) -> & T { & self . 0 } # [doc = " Returns a mutable reference to the contained IO object."] pub fn get_mut (& mut self) -> & mut T { & mut self . 0 } # [doc = " Consumes self and returns the contained IO object."] pub fn into_inner (self) -> T { self . 0 } }
};
}
