// Generated macro for Types (struct)
macro_rules! Depcrate_typesTypes {
() => {
// Module: crate::types
// Provides: {"Types"}
// Dependencies: {}
# [doc = " Types is a file type matcher."] # [derive (Clone , Debug)] pub struct Types { # [doc = " All of the file type definitions, sorted lexicographically by name."] defs : Vec < FileTypeDef > , # [doc = " All of the selections made by the user."] selections : Vec < Selection < FileTypeDef > > , # [doc = " Whether there is at least one Selection::Select in our selections."] # [doc = " When this is true, a Match::None is converted to Match::Ignore."] has_selected : bool , # [doc = " A mapping from glob index in the set to two indices. The first is an"] # [doc = " index into `selections` and the second is an index into the"] # [doc = " corresponding file type definition's list of globs."] glob_to_selection : Vec < (usize , usize) > , # [doc = " The set of all glob selections, used for actual matching."] set : GlobSet , # [doc = " Temporary storage for globs that match."] matches : Arc < Pool < Vec < usize > > > , }
};
}
