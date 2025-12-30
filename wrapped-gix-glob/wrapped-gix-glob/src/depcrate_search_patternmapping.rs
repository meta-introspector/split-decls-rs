// Generated macro for Mapping (struct)
macro_rules! Depcrate_search_patternMapping {
() => {
// Module: crate::search::pattern
// Provides: {"Mapping"}
// Dependencies: {}
# [doc = " An association of a pattern with its value, along with a sequence number providing a sort order in relation to its peers."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] pub struct Mapping < T > { # [doc = " The pattern itself, like `/target/*`"] pub pattern : crate :: Pattern , # [doc = " The value associated with the pattern."] pub value : T , # [doc = " Typically the line number in the file the pattern was parsed from."] pub sequence_number : usize , }
};
}
