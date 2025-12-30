// Generated macro for impl_93 (impl)
macro_rules! Depcrate_tree_with_rewrites_changeimpl_93 {
() => {
// Module: crate::tree_with_rewrites::change
// Provides: {"impl_93"}
// Dependencies: {}
# [doc = " Lifecycle"] impl ChangeRef < '_ > { # [doc = " Copy this instance into a fully-owned version"] pub fn into_owned (self) -> Change { match self { ChangeRef :: Addition { location , entry_mode , id , relation , } => Change :: Addition { location : location . to_owned () , entry_mode , id , relation , } , ChangeRef :: Deletion { location , entry_mode , id , relation , } => Change :: Deletion { location : location . to_owned () , entry_mode , id , relation , } , ChangeRef :: Modification { location , previous_entry_mode , previous_id , entry_mode , id , } => Change :: Modification { location : location . to_owned () , previous_entry_mode , previous_id , entry_mode , id , } , ChangeRef :: Rewrite { source_location , source_relation , source_entry_mode , source_id , diff , entry_mode , id , location , relation , copy , } => Change :: Rewrite { source_location : source_location . to_owned () , source_relation , source_entry_mode , source_id , diff , entry_mode , id , location : location . to_owned () , relation , copy , } , } } }
};
}
