// Generated macro for impl_1550 (impl)
macro_rules! Depcrate_build_bytesimpl_1550 {
() => {
// Module: crate::build::bytes
// Provides: {"impl_1550"}
// Dependencies: {}
impl < 'a > ByteString < 'a > { # [doc = " Acquire a mutable reference to the bytes."] # [doc = ""] # [doc = " Clones the bytes if they are shared."] pub fn to_mut (& mut self) -> & mut Vec < u8 > { self . 0 . to_mut () } # [doc = " Get the bytes as a slice."] pub fn as_slice (& self) -> & [u8] { self . 0 . as_ref () } }
};
}
