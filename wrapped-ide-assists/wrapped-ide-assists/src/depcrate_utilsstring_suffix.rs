// Generated macro for string_suffix (function)
macro_rules! Depcrate_utilsstring_suffix {
() => {
// Module: crate::utils
// Provides: {"string_suffix"}
// Dependencies: {}
# [doc = " Calculate the string literal suffix length"] pub (crate) fn string_suffix (s : & str) -> Option < & str > { s . rfind (['"' , '\'' , '#']) . map (| i | & s [i + 1 ..]) }
};
}
