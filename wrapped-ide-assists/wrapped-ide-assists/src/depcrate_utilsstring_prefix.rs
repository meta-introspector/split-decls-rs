// Generated macro for string_prefix (function)
macro_rules! Depcrate_utilsstring_prefix {
() => {
// Module: crate::utils
// Provides: {"string_prefix"}
// Dependencies: {}
# [doc = " Calculate the string literal prefix length"] pub (crate) fn string_prefix (s : & str) -> Option < & str > { s . split_once (['"' , '\'' , '#']) . map (| (prefix , _) | prefix) }
};
}
