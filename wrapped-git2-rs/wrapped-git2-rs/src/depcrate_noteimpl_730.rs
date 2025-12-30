// Generated macro for impl_730 (impl)
macro_rules! Depcrate_noteimpl_730 {
() => {
// Module: crate::note
// Provides: {"impl_730"}
// Dependencies: {}
impl < 'repo > Drop for Notes < 'repo > { fn drop (& mut self) { unsafe { raw :: git_note_iterator_free (self . raw) ; } } }
};
}
