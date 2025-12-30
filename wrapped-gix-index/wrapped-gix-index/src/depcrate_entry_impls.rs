// Generated macro for _impls (module)
macro_rules! Depcrate_entry_impls {
() => {
// Module: crate::entry
// Provides: {"_impls"}
// Dependencies: {}
mod _impls { use std :: cmp :: Ordering ; use bstr :: BStr ; use gix_object :: tree :: EntryKind ; use crate :: { entry , Entry , State } ; impl From < EntryKind > for entry :: Mode { fn from (value : EntryKind) -> Self { match value { EntryKind :: Tree => entry :: Mode :: DIR , EntryKind :: Blob => entry :: Mode :: FILE , EntryKind :: BlobExecutable => entry :: Mode :: FILE_EXECUTABLE , EntryKind :: Link => entry :: Mode :: SYMLINK , EntryKind :: Commit => entry :: Mode :: COMMIT , } } } impl Entry { # [doc = " Compare one entry to another by their path, by comparing only their common path portion byte by byte, then resorting to"] # [doc = " entry length and stage."] pub fn cmp (& self , other : & Self , state : & State) -> Ordering { let lhs = self . path (state) ; let rhs = other . path (state) ; Entry :: cmp_filepaths (lhs , rhs) . then_with (| | self . stage () . cmp (& other . stage ())) } # [doc = " Compare one entry to another by their path, by comparing only their common path portion byte by byte, then resorting to"] # [doc = " entry length."] pub fn cmp_filepaths (a : & BStr , b : & BStr) -> Ordering { let common_len = a . len () . min (b . len ()) ; a [.. common_len] . cmp (& b [.. common_len]) . then_with (| | a . len () . cmp (& b . len ())) } } }
};
}
