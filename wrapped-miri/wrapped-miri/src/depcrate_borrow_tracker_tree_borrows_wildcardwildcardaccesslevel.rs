// Generated macro for WildcardAccessLevel (enum)
macro_rules! Depcrate_borrow_tracker_tree_borrows_wildcardWildcardAccessLevel {
() => {
// Module: crate::borrow_tracker::tree_borrows::wildcard
// Provides: {"WildcardAccessLevel"}
// Dependencies: {}
# [doc = " Represents the maximum access level that is possible."] # [doc = ""] # [doc = " Note that we derive Ord and PartialOrd, so the order in which variants are listed below matters:"] # [doc = " None < Read < Write. Do not change that order."] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Default)] pub enum WildcardAccessLevel { # [default] None , Read , Write , }
};
}
