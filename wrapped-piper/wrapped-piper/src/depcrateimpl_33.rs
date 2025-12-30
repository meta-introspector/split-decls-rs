// Generated macro for impl_33 (impl)
macro_rules! Depcrateimpl_33 {
() => {
// Module: crate
// Provides: {"impl_33"}
// Dependencies: {}
# [cfg (feature = "std")] impl < W : Write > WriteLike for W { type Error = io :: Error ; fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { Write :: write (self , buf) } }
};
}
