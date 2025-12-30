// Generated macro for add_group_separators (function)
macro_rules! Depcrate_utilsadd_group_separators {
() => {
// Module: crate::utils
// Provides: {"add_group_separators"}
// Dependencies: {}
pub (crate) fn add_group_separators (s : & str , group_size : usize) -> String { let mut chars = Vec :: new () ; for (i , ch) in s . chars () . filter (| & ch | ch != '_') . rev () . enumerate () { if i > 0 && i % group_size == 0 && ch != '-' { chars . push ('_') ; } chars . push (ch) ; } chars . into_iter () . rev () . collect () }
};
}
