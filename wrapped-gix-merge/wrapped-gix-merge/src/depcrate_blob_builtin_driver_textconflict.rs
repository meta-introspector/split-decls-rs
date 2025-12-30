// Generated macro for Conflict (enum)
macro_rules! Depcrate_blob_builtin_driver_textConflict {
() => {
// Module: crate::blob::builtin_driver::text
// Provides: {"Conflict"}
// Dependencies: {}
# [doc = " What to do to resolve a conflict."] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum Conflict { # [doc = " Keep the conflict by marking it in the source file."] Keep { # [doc = " How to visualize conflicts in merged files."] style : ConflictStyle , # [doc = " The amount of markers to draw, defaults to 7, i.e. `<<<<<<<`"] marker_size : NonZeroU8 , } , # [doc = " Chose our side to resolve a conflict."] ResolveWithOurs , # [doc = " Chose their side to resolve a conflict."] ResolveWithTheirs , # [doc = " Place our and their lines one after another, in any order"] ResolveWithUnion , }
};
}
