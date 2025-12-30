// Generated macro for impl_368 (impl)
macro_rules! Depcrate_re_unicodeimpl_368 {
() => {
// Module: crate::re_unicode
// Provides: {"impl_368"}
// Dependencies: {}
# [doc = " Equality comparison is based on the original string. It is possible that"] # [doc = " different regular expressions have the same matching behavior, but are"] # [doc = " still compared unequal. For example, `\\d+` and `\\d\\d*` match the same set"] # [doc = " of strings, but are not considered equal."] impl PartialEq for Regex { fn eq (& self , other : & Regex) -> bool { self . as_str () == other . as_str () } }
};
}
