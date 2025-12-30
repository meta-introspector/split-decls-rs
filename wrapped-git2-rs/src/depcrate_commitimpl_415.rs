// Generated macro for impl_415 (impl)
macro_rules! Depcrate_commitimpl_415 {
() => {
// Module: crate::commit
// Provides: {"impl_415"}
// Dependencies: {}
impl < 'repo > Binding for Commit < 'repo > { type Raw = * mut raw :: git_commit ; unsafe fn from_raw (raw : * mut raw :: git_commit) -> Commit < 'repo > { Commit { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_commit { self . raw } }
};
}
