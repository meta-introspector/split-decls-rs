// Generated macro for maybe_filter_value (function)
macro_rules! Depcrate_codepointtrie_cptriemaybe_filter_value {
() => {
// Module: crate::codepointtrie::cptrie
// Provides: {"maybe_filter_value"}
// Dependencies: {}
# [doc = " Helper function used by [`get_range`]. Converts occurrences of trie's null"] # [doc = " value into the provided `null_value`."] # [doc = ""] # [doc = " Note: the ICU version of this helper function uses a `ValueFilter` function"] # [doc = " to apply a transform on a non-null value. But currently, this implementation"] # [doc = " stops short of that functionality, and instead leaves the non-null trie value"] # [doc = " untouched. This is equivalent to having a `ValueFilter` function that is the"] # [doc = " identity function."] fn maybe_filter_value < T : TrieValue > (value : T , trie_null_value : T , null_value : T) -> T { if value == trie_null_value { null_value } else { value } }
};
}
