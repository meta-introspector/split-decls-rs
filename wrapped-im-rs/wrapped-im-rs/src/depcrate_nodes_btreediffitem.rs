// Generated macro for DiffItem (enum)
macro_rules! Depcrate_nodes_btreeDiffItem {
() => {
// Module: crate::nodes::btree
// Provides: {"DiffItem"}
// Dependencies: {}
# [doc = " A description of a difference between two ordered sets."] # [derive (PartialEq , Eq , Debug)] pub enum DiffItem < 'a , A > { # [doc = " This value has been added to the new set."] Add (& 'a A) , # [doc = " This value has been changed between the two sets."] Update { # [doc = " The old value."] old : & 'a A , # [doc = " The new value."] new : & 'a A , } , # [doc = " This value has been removed from the new set."] Remove (& 'a A) , }
};
}
