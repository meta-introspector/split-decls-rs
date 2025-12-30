// Generated macro for Change (enum)
macro_rules! Depcrate_tree_visitChange {
() => {
// Module: crate::tree::visit
// Provides: {"Change"}
// Dependencies: {}
# [doc = " Represents any possible change in order to turn one tree into another."] # [derive (Debug , Clone , PartialOrd , PartialEq , Ord , Eq , Hash)] pub enum Change { # [doc = " An entry was added, like the addition of a file or directory."] Addition { # [doc = " The mode of the added entry."] entry_mode : tree :: EntryMode , # [doc = " The object id of the added entry."] oid : ObjectId , # [doc = " Possibly associate this change with another for hierarchical rename tracking."] relation : Option < Relation > , } , # [doc = " An entry was deleted, like the deletion of a file or directory."] Deletion { # [doc = " The mode of the deleted entry."] entry_mode : tree :: EntryMode , # [doc = " The object id of the deleted entry."] oid : ObjectId , # [doc = " Possibly associate this change with another for hierarchical rename tracking."] relation : Option < Relation > , } , # [doc = " An entry was modified, e.g. changing the contents of a file adjusts its object id and turning"] # [doc = " a file into a symbolic link adjusts its mode."] Modification { # [doc = " The mode of the entry before the modification."] previous_entry_mode : tree :: EntryMode , # [doc = " The object id of the entry before the modification."] previous_oid : ObjectId , # [doc = " The mode of the entry after the modification."] entry_mode : tree :: EntryMode , # [doc = " The object id after the modification."] oid : ObjectId , } , }
};
}
