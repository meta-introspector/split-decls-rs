// Generated macro for ChangeKind (enum)
macro_rules! Depcrate_rewrites_trackerChangeKind {
() => {
// Module: crate::rewrites::tracker
// Provides: {"ChangeKind"}
// Dependencies: {}
# [doc = " The kind of a change."] # [derive (Debug , Copy , Clone , Ord , PartialOrd , PartialEq , Eq)] pub enum ChangeKind { # [doc = " The change represents the *deletion* of an item."] Deletion , # [doc = " The change represents the *modification* of an item."] Modification , # [doc = " The change represents the *addition* of an item."] Addition , }
};
}
