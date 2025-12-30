// Generated macro for impl_1285 (impl)
macro_rules! Depcrate_treeimpl_1285 {
() => {
// Module: crate::tree
// Provides: {"impl_1285"}
// Dependencies: {}
impl < 'repo > Binding for Tree < 'repo > { type Raw = * mut raw :: git_tree ; unsafe fn from_raw (raw : * mut raw :: git_tree) -> Tree < 'repo > { Tree { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_tree { self . raw } }
};
}
