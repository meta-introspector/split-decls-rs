// Generated macro for Entry (struct)
macro_rules! Depcrate_treeEntry {
() => {
// Module: crate::tree
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " An entry in a [`Tree`], similar to an entry in a directory."] # [derive (PartialEq , Eq , Debug , Hash , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Entry { # [doc = " The kind of object to which `oid` is pointing to."] pub mode : EntryMode , # [doc = " The name of the file in the parent tree."] pub filename : BString , # [doc = " The id of the object representing the entry."] pub oid : gix_hash :: ObjectId , }
};
}
