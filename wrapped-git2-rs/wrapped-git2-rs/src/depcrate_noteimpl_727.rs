// Generated macro for impl_727 (impl)
macro_rules! Depcrate_noteimpl_727 {
() => {
// Module: crate::note
// Provides: {"impl_727"}
// Dependencies: {}
impl < 'repo > Drop for Note < 'repo > { fn drop (& mut self) { unsafe { raw :: git_note_free (self . raw) ; } } }
};
}
