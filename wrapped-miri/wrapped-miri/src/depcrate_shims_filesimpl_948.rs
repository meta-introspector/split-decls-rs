// Generated macro for impl_948 (impl)
macro_rules! Depcrate_shims_filesimpl_948 {
() => {
// Module: crate::shims::files
// Provides: {"impl_948"}
// Dependencies: {}
impl < T : ? Sized > WeakFileDescriptionRef < T > { pub fn upgrade (& self) -> Option < FileDescriptionRef < T > > { self . 0 . upgrade () . map (FileDescriptionRef) } }
};
}
