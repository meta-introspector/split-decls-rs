// Generated macro for impl_120 (impl)
macro_rules! Depcrate_decodeimpl_120 {
() => {
// Module: crate::decode
// Provides: {"impl_120"}
// Dependencies: {}
impl < 'a , T : AsRef < [u8] > + ? Sized > ReadRefReader < 'a , T > { # [inline] fn new (rd : & 'a T) -> Self { Self { whole_slice : rd , buf : rd . as_ref () , } } }
};
}
