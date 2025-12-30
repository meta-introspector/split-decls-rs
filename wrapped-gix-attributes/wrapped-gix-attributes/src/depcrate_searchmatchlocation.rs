// Generated macro for MatchLocation (struct)
macro_rules! Depcrate_searchMatchLocation {
() => {
// Module: crate::search
// Provides: {"MatchLocation"}
// Dependencies: {}
# [doc = " Describes in which what file and line the match was found."] # [derive (Clone , PartialEq , Eq , Debug , Hash , Ord , PartialOrd)] pub struct MatchLocation < 'a > { # [doc = " The path to the source from which the pattern was loaded, or `None` if it was specified by other means."] pub source : Option < & 'a std :: path :: Path > , # [doc = " The line at which the pattern was found in its `source` file, or the occurrence in which it was provided."] pub sequence_number : usize , }
};
}
