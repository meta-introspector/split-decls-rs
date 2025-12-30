// Generated macro for entry_from_raw_const (function)
macro_rules! Depcrate_treeentry_from_raw_const {
() => {
// Module: crate::tree
// Provides: {"entry_from_raw_const"}
// Dependencies: {}
# [doc = " Create a new tree entry from the raw pointer provided."] # [doc = ""] # [doc = " The lifetime of the entry is tied to the tree provided and the function"] # [doc = " is unsafe because the validity of the pointer cannot be guaranteed."] pub unsafe fn entry_from_raw_const < 'tree > (raw : * const raw :: git_tree_entry) -> TreeEntry < 'tree > { TreeEntry { raw : raw as * mut raw :: git_tree_entry , owned : false , _marker : marker :: PhantomData , } }
};
}
