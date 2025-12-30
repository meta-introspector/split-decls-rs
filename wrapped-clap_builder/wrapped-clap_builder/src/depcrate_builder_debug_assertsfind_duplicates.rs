// Generated macro for find_duplicates (function)
macro_rules! Depcrate_builder_debug_assertsfind_duplicates {
() => {
// Module: crate::builder::debug_asserts
// Provides: {"find_duplicates"}
// Dependencies: {}
# [doc = " Find duplicates in a sorted array."] # [doc = ""] # [doc = " The algorithm is simple: the array is sorted, duplicates"] # [doc = " must be placed next to each other, we can check only adjacent elements."] fn find_duplicates < T : PartialEq > (slice : & [T]) -> impl Iterator < Item = (& T , & T) > { slice . windows (2) . filter_map (| w | { if w [0] == w [1] { Some ((& w [0] , & w [1])) } else { None } }) }
};
}
