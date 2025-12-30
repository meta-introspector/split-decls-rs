// Generated macro for Change (enum)
macro_rules! Depcrate_tree_recorderChange {
() => {
// Module: crate::tree::recorder
// Provides: {"Change"}
// Dependencies: {}
# [doc = " A Change as observed by a call to [`visit(…)`](Visit::visit()), enhanced with the path affected by the change."] # [doc = " Its similar to [`visit::Change`] but includes the path that changed."] # [derive (Clone , Debug , PartialEq , Eq)] # [allow (missing_docs)] pub enum Change { Addition { entry_mode : tree :: EntryMode , oid : ObjectId , path : BString , relation : Option < Relation > , } , Deletion { entry_mode : tree :: EntryMode , oid : ObjectId , path : BString , relation : Option < Relation > , } , Modification { previous_entry_mode : tree :: EntryMode , previous_oid : ObjectId , entry_mode : tree :: EntryMode , oid : ObjectId , path : BString , } , }
};
}
