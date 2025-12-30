// Generated macro for impl_1295 (impl)
macro_rules! Depcrate_treeimpl_1295 {
() => {
// Module: crate::tree
// Provides: {"impl_1295"}
// Dependencies: {}
impl < 'a > Ord for TreeEntry < 'a > { fn cmp (& self , other : & TreeEntry < 'a >) -> Ordering { c_cmp_to_ordering (unsafe { raw :: git_tree_entry_cmp (& * self . raw () , & * other . raw ()) }) } }
};
}
