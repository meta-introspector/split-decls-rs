// Generated macro for impl_998 (impl)
macro_rules! Depcrate_remoteimpl_998 {
() => {
// Module: crate::remote
// Provides: {"impl_998"}
// Dependencies: {}
impl < 'repo > Binding for Remote < 'repo > { type Raw = * mut raw :: git_remote ; unsafe fn from_raw (raw : * mut raw :: git_remote) -> Remote < 'repo > { Remote { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_remote { self . raw } }
};
}
