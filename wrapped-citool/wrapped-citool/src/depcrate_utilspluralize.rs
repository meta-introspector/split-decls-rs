// Generated macro for pluralize (function)
macro_rules! Depcrate_utilspluralize {
() => {
// Module: crate::utils
// Provides: {"pluralize"}
// Dependencies: {}
pub fn pluralize (text : & str , count : usize) -> String { if count == 1 { text . to_string () } else { format ! ("{text}s") } }
};
}
