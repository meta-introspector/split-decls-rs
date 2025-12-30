// Generated macro for WildcardAccessRelatedness (enum)
macro_rules! Depcrate_borrow_tracker_tree_borrows_wildcardWildcardAccessRelatedness {
() => {
// Module: crate::borrow_tracker::tree_borrows::wildcard
// Provides: {"WildcardAccessRelatedness"}
// Dependencies: {}
# [doc = " Where the access happened relative to the current node."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum WildcardAccessRelatedness { # [doc = " The access definitively happened through a local node."] LocalAccess , # [doc = " The access definitively happened through a foreign node."] ForeignAccess , # [doc = " We do not know if the access is foreign or local."] EitherAccess , }
};
}
