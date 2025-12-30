// Generated macro for split_on_equals (function)
macro_rules! Depcrate_options_parsersplit_on_equals {
() => {
// Module: crate::options::parser
// Provides: {"split_on_equals"}
// Dependencies: {}
# [doc = " Splits a string on its `=` character, returning the two substrings on"] # [doc = " either side. Returns `None` if there’s no equals or a string is missing."] fn split_on_equals (input : & OsStr) -> Option < (& OsStr , & OsStr) > { if let Some (index) = os_str_to_bytes (input) . iter () . position (| elem | * elem == b'=') { let (before , after) = os_str_to_bytes (input) . split_at (index) ; if ! before . is_empty () && after . len () >= 2 { return Some ((bytes_to_os_str (before) , bytes_to_os_str (& after [1 ..]))) ; } } None }
};
}
