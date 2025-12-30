// Generated macro for EntryRef (struct)
macro_rules! Depcrate_object_treeEntryRef {
() => {
// Module: crate::object::tree
// Provides: {"EntryRef"}
// Dependencies: {}
# [doc = " An entry within a tree"] pub struct EntryRef < 'repo , 'a > { # [doc = " The actual entry ref we are wrapping."] pub inner : gix_object :: tree :: EntryRef < 'a > , # [doc = " The owning repository."] pub repo : & 'repo Repository , }
};
}
