// Generated macro for Entry (struct)
macro_rules! Depcrate_tree_recorderEntry {
() => {
// Module: crate::tree::recorder
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " An owned entry as observed by a call to [`visit_(tree|nontree)(…)`][Visit::visit_tree()], enhanced with the full path to it."] # [doc = " Otherwise similar to [`gix_object::tree::EntryRef`]."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct Entry { # [doc = " The kind of entry, similar to entries in a unix directory tree."] pub mode : tree :: EntryMode , # [doc = " The full path to the entry. A root entry would be `d`, and a file `a` within the directory would be `d/a`."] # [doc = ""] # [doc = " This is independent of the platform and the path separators actually used there."] pub filepath : BString , # [doc = " The id of the entry which can be used to locate it in an object database."] pub oid : ObjectId , }
};
}
