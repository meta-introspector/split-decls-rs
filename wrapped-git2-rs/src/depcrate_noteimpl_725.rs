// Generated macro for impl_725 (impl)
macro_rules! Depcrate_noteimpl_725 {
() => {
// Module: crate::note
// Provides: {"impl_725"}
// Dependencies: {}
impl < 'repo > Binding for Note < 'repo > { type Raw = * mut raw :: git_note ; unsafe fn from_raw (raw : * mut raw :: git_note) -> Note < 'repo > { Note { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_note { self . raw } }
};
}
