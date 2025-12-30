// Generated macro for impl_728 (impl)
macro_rules! Depcrate_noteimpl_728 {
() => {
// Module: crate::note
// Provides: {"impl_728"}
// Dependencies: {}
impl < 'repo > Binding for Notes < 'repo > { type Raw = * mut raw :: git_note_iterator ; unsafe fn from_raw (raw : * mut raw :: git_note_iterator) -> Notes < 'repo > { Notes { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_note_iterator { self . raw } }
};
}
