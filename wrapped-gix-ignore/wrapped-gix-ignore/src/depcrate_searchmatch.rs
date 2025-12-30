// Generated macro for Match (struct)
macro_rules! Depcrate_searchMatch {
() => {
// Module: crate::search
// Provides: {"Match"}
// Dependencies: {}
# [doc = " Describes a matching pattern within a search for ignored paths."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] pub struct Match < 'a > { # [doc = " The glob pattern itself, like `/target/*`."] pub pattern : & 'a gix_glob :: Pattern , # [doc = " The path to the source from which the pattern was loaded, or `None` if it was specified by other means."] pub source : Option < & 'a Path > , # [doc = " The kind of pattern this match represents."] pub kind : crate :: Kind , # [doc = " The line at which the pattern was found in its `source` file, or the occurrence in which it was provided."] pub sequence_number : usize , }
};
}
