// Generated macro for tests (module)
macro_rules! Depcrate_tree_functiontests {
() => {
// Module: crate::tree::function
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: cmp :: Ordering ; use gix_object :: tree :: EntryKind ; use super :: * ; # [test] fn compare_select_samples () { let null = gix_hash :: ObjectId :: null (gix_hash :: Kind :: Sha1) ; let actual = compare (& EntryRef { mode : EntryKind :: Blob . into () , filename : "plumbing-cli.rs" . into () , oid : & null , } , & EntryRef { mode : EntryKind :: Tree . into () , filename : "plumbing" . into () , oid : & null , } ,) ; assert_eq ! (actual , Ordering :: Less) ; let actual = compare (& EntryRef { mode : EntryKind :: Tree . into () , filename : "plumbing-cli.rs" . into () , oid : & null , } , & EntryRef { mode : EntryKind :: Blob . into () , filename : "plumbing" . into () , oid : & null , } ,) ; assert_eq ! (actual , Ordering :: Greater) ; } }
};
}
