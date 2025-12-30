// Generated macro for Utf8SuffixEntry (struct)
macro_rules! Depcrate_nfa_thompson_mapUtf8SuffixEntry {
() => {
// Module: crate::nfa::thompson::map
// Provides: {"Utf8SuffixEntry"}
// Dependencies: {}
# [doc = " An entry in this map."] # [derive (Clone , Debug , Default)] struct Utf8SuffixEntry { # [doc = " The version of the map used to produce this entry. If this entry's"] # [doc = " version does not match the current version of the map, then the map"] # [doc = " should behave as if this entry does not exist."] version : u16 , # [doc = " The key, which consists of a transition in a particular state."] key : Utf8SuffixKey , # [doc = " The identifier that the transition in the key maps to."] val : StateID , }
};
}
