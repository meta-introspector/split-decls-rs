// Generated macro for Utf8SuffixKey (struct)
macro_rules! Depcrate_nfa_thompson_mapUtf8SuffixKey {
() => {
// Module: crate::nfa::thompson::map
// Provides: {"Utf8SuffixKey"}
// Dependencies: {}
# [doc = " A key that uniquely identifies an NFA state. It is a triple that represents"] # [doc = " a transition from one state for a particular byte range."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct Utf8SuffixKey { pub from : StateID , pub start : u8 , pub end : u8 , }
};
}
