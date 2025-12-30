// Generated macro for flagsplit (function)
macro_rules! Depcrate_utilflagsplit {
() => {
// Module: crate::util
// Provides: {"flagsplit"}
// Dependencies: {}
pub fn flagsplit (flags : & str) -> Vec < String > { flags . split (' ') . map (str :: trim) . filter (| s | ! s . is_empty ()) . map (str :: to_string) . collect () }
};
}
