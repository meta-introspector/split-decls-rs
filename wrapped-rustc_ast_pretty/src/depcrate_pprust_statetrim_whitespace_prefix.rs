// Generated macro for trim_whitespace_prefix (function)
macro_rules! Depcrate_pprust_statetrim_whitespace_prefix {
() => {
// Module: crate::pprust::state
// Provides: {"trim_whitespace_prefix"}
// Dependencies: {}
fn trim_whitespace_prefix (s : & str , col : CharPos) -> & str { let len = s . len () ; match all_whitespace (s , col) { Some (col) => { if col < len { & s [col ..] } else { "" } } None => s , } }
};
}
