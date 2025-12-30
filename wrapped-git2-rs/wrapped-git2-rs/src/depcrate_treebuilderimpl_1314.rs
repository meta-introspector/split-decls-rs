// Generated macro for impl_1314 (impl)
macro_rules! Depcrate_treebuilderimpl_1314 {
() => {
// Module: crate::treebuilder
// Provides: {"impl_1314"}
// Dependencies: {}
impl < 'repo > Binding for TreeBuilder < 'repo > { type Raw = * mut raw :: git_treebuilder ; unsafe fn from_raw (raw : * mut raw :: git_treebuilder) -> TreeBuilder < 'repo > { TreeBuilder { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_treebuilder { self . raw } }
};
}
