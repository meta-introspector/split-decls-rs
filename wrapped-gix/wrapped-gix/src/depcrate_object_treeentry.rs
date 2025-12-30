// Generated macro for Entry (struct)
macro_rules! Depcrate_object_treeEntry {
() => {
// Module: crate::object::tree
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " An entry in a [`Tree`], similar to an entry in a directory."] # [derive (PartialEq , Debug , Clone)] pub struct Entry < 'repo > { pub (crate) inner : gix_object :: tree :: Entry , # [doc = " The owning repository."] pub repo : & 'repo crate :: Repository , }
};
}
