// Generated macro for TreeBuilder (struct)
macro_rules! Depcrate_treebuilderTreeBuilder {
() => {
// Module: crate::treebuilder
// Provides: {"TreeBuilder"}
// Dependencies: {}
# [doc = " Constructor for in-memory trees (low-level)"] # [doc = ""] # [doc = " You probably want to use [`build::TreeUpdateBuilder`] instead."] # [doc = ""] # [doc = " This is the more raw of the two tree update facilities.  It"] # [doc = " handles only one level of a nested tree structure at a time.  Each"] # [doc = " path passed to `insert` etc. must be a single component."] # [doc = ""] # [doc = " [`build::TreeUpdateBuilder`]: crate::build::TreeUpdateBuilder"] pub struct TreeBuilder < 'repo > { raw : * mut raw :: git_treebuilder , _marker : marker :: PhantomData < & 'repo Repository > , }
};
}
