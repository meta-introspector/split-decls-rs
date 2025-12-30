// Generated macro for impl_1117 (impl)
macro_rules! Depcrate_shims_filesimpl_1117 {
() => {
// Module: crate::shims::files
// Provides: {"impl_1117"}
// Dependencies: {}
impl < T : ? Sized > WeakFileDescriptionRef < T > { pub fn upgrade (& self) -> Option < FileDescriptionRef < T > > { self . 0 . upgrade () . map (FileDescriptionRef) } }
};
}
