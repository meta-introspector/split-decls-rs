// Generated macro for EntryRef (struct)
macro_rules! Depcrate_treeEntryRef {
() => {
// Module: crate::tree
// Provides: {"EntryRef"}
// Dependencies: {}
# [doc = " An element of a [`TreeRef`][crate::TreeRef::entries]."] # [derive (PartialEq , Eq , Debug , Hash , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct EntryRef < 'a > { # [doc = " The kind of object to which `oid` is pointing."] pub mode : tree :: EntryMode , # [doc = " The name of the file in the parent tree."] pub filename : & 'a BStr , # [doc = " The id of the object representing the entry."] # [cfg_attr (feature = "serde" , serde (borrow))] pub oid : & 'a gix_hash :: oid , }
};
}
