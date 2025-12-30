// Generated macro for NFA (struct)
macro_rules! Depcrate_nfaNFA {
() => {
// Module: crate::nfa
// Provides: {"NFA"}
// Dependencies: {}
# [derive (Clone)] pub (crate) struct NFA { # [doc = " The pattern string this NFA was generated from."] # [doc = ""] # [doc = " We put it here for lack of a better place to put it. ¯\\_(ツ)_/¯"] pattern : String , # [doc = " The states that make up this NFA."] states : Vec < State > , # [doc = " The ID of the start state."] start : StateID , # [doc = " Whether this NFA can only match at the beginning of a haystack."] is_start_anchored : bool , # [doc = " Whether this NFA can match the empty string."] is_match_empty : bool , # [doc = " If every match has the same number of matching capture groups, then"] # [doc = " this corresponds to the number of groups."] static_explicit_captures_len : Option < usize > , # [doc = " A map from capture group name to its corresponding index."] cap_name_to_index : CaptureNameMap , # [doc = " A map from capture group index to the corresponding name, if one"] # [doc = " exists."] cap_index_to_name : Vec < Option < Arc < str > > > , # [doc = " Heap memory used indirectly by NFA states and other things (like the"] # [doc = " various capturing group representations above). Since each state"] # [doc = " might use a different amount of heap, we need to keep track of this"] # [doc = " incrementally."] memory_extra : usize , }
};
}
