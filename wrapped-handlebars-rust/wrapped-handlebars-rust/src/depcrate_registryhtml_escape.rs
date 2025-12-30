// Generated macro for html_escape (function)
macro_rules! Depcrate_registryhtml_escape {
() => {
// Module: crate::registry
// Provides: {"html_escape"}
// Dependencies: {}
# [doc = " The default *escape fn* replaces the characters `&\"<>`"] # [doc = " with the equivalent html / xml entities."] pub fn html_escape (data : & str) -> String { str :: escape_html (data) }
};
}
