// Generated macro for impl_1292 (impl)
macro_rules! Depcrate_treeimpl_1292 {
() => {
// Module: crate::tree
// Provides: {"impl_1292"}
// Dependencies: {}
impl < 'a > Binding for TreeEntry < 'a > { type Raw = * mut raw :: git_tree_entry ; unsafe fn from_raw (raw : * mut raw :: git_tree_entry) -> TreeEntry < 'a > { TreeEntry { raw , owned : true , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_tree_entry { self . raw } }
};
}
