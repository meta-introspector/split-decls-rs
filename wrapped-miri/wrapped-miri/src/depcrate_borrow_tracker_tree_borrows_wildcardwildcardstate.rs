// Generated macro for WildcardState (struct)
macro_rules! Depcrate_borrow_tracker_tree_borrows_wildcardWildcardState {
() => {
// Module: crate::borrow_tracker::tree_borrows::wildcard
// Provides: {"WildcardState"}
// Dependencies: {}
# [doc = " State per location per node keeping track of where relative to this"] # [doc = " node exposed nodes are and what access permissions they have."] # [doc = ""] # [doc = " Designed to be completely determined by its parent, siblings and"] # [doc = " direct children's max_local_access/max_foreign_access."] # [derive (Clone , Default , PartialEq , Eq)] pub struct WildcardState { # [doc = " How many of this node's direct children have `max_local_access()==Write`."] child_writes : u16 , # [doc = " How many of this node's direct children have `max_local_access()>=Read`."] child_reads : u16 , # [doc = " The maximum access level that could happen from an exposed node"] # [doc = " that is foreign to this node."] # [doc = ""] # [doc = " This is calculated as the `max()` of the parent's `max_foreign_access`,"] # [doc = " `exposed_as` and the siblings' `max_local_access()`."] max_foreign_access : WildcardAccessLevel , # [doc = " At what access level this node itself is exposed."] exposed_as : WildcardAccessLevel , }
};
}
