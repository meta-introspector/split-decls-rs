// Generated macro for visit (module)
macro_rules! Depcrate_rewrites_trackervisit {
() => {
// Module: crate::rewrites::tracker
// Provides: {"visit"}
// Dependencies: {}
# [doc = " A module with types used in the user-callback in [Tracker::emit()](crate::rewrites::Tracker::emit())."] pub mod visit { use bstr :: BStr ; use gix_object :: tree :: EntryMode ; use crate :: blob :: DiffLineStats ; # [doc = " The source of a rewrite, rename or copy."] # [derive (Debug , Clone , PartialEq , PartialOrd)] pub struct Source < 'a , T > { # [doc = " The kind of entry."] pub entry_mode : EntryMode , # [doc = " The hash of the state of the source as seen in the object database."] pub id : gix_hash :: ObjectId , # [doc = " Further specify what kind of source this is."] pub kind : SourceKind , # [doc = " The repository-relative location of this entry."] pub location : & 'a BStr , # [doc = " The change that was registered as source."] pub change : & 'a T , # [doc = " If this is a rewrite, indicate how many lines would need to change to turn this source into the destination."] pub diff : Option < DiffLineStats > , } # [doc = " Further identify the kind of [Source]."] # [derive (Debug , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum SourceKind { # [doc = " This is the source of an entry that was renamed, as `source` was renamed to `destination`."] Rename , # [doc = " This is the source of a copy, as `source` was copied into `destination`."] Copy , } # [doc = " A change along with a location."] # [derive (Debug , Clone)] pub struct Destination < 'a , T : Clone > { # [doc = " The change at the given `location`."] pub change : T , # [doc = " The repository-relative location of this destination."] pub location : & 'a BStr , } }
};
}
