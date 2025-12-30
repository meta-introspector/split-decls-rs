// Generated macro for impl_403 (impl)
macro_rules! Depcrate_imageimpl_403 {
() => {
// Module: crate::image
// Provides: {"impl_403"}
// Dependencies: {}
impl < I > Deref for SubImage < I > where I : Deref , { type Target = SubImageInner < I > ; fn deref (& self) -> & Self :: Target { & self . inner } }
};
}
