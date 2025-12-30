// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
# [cfg (feature = "std")] impl < R : Read > ReadLike for R { type Error = io :: Error ; fn read (& mut self , buf : & mut [u8]) -> Result < usize , Self :: Error > { Read :: read (self , buf) } }
};
}
