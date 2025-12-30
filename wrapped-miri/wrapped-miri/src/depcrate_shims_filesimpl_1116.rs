// Generated macro for impl_1116 (impl)
macro_rules! Depcrate_shims_filesimpl_1116 {
() => {
// Module: crate::shims::files
// Provides: {"impl_1116"}
// Dependencies: {}
impl < T : ? Sized > FileDescriptionRef < T > { pub fn downgrade (this : & Self) -> WeakFileDescriptionRef < T > { WeakFileDescriptionRef (Rc :: downgrade (& this . 0)) } }
};
}
