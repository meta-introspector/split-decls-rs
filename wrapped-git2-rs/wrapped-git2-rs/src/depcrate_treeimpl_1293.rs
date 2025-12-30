// Generated macro for impl_1293 (impl)
macro_rules! Depcrate_treeimpl_1293 {
() => {
// Module: crate::tree
// Provides: {"impl_1293"}
// Dependencies: {}
impl < 'a > Clone for TreeEntry < 'a > { fn clone (& self) -> TreeEntry < 'a > { let mut ret = ptr :: null_mut () ; unsafe { assert_eq ! (raw :: git_tree_entry_dup (& mut ret , &* self . raw ()) , 0) ; Binding :: from_raw (ret) } } }
};
}
