// Generated macro for to_title_case (function)
macro_rules! Depcrate_configto_title_case {
() => {
// Module: crate::config
// Provides: {"to_title_case"}
// Dependencies: {}
# [doc = " Translate a field name to a title case string suitable for use in the category names on the"] # [doc = " vscode settings page."] # [doc = ""] # [doc = " First letter of word should be uppercase, if an uppercase letter is encountered, add a space"] # [doc = " before it e.g. \"fooBar\" -> \"Foo Bar\", \"fooBarBaz\" -> \"Foo Bar Baz\", \"foo\" -> \"Foo\""] # [doc = ""] # [doc = " This likely should be in stdx (or just use heck instead), but it doesn't handle any edge cases"] # [doc = " and is intentionally simple."] fn to_title_case (s : & str) -> String { let mut result = String :: with_capacity (s . len ()) ; let mut chars = s . chars () ; if let Some (first) = chars . next () { result . push (first . to_ascii_uppercase ()) ; for c in chars { if c . is_uppercase () { result . push (' ') ; } result . push (c) ; } } result }
};
}
