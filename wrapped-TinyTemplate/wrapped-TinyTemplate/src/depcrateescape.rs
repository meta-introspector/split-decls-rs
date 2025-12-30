// Generated macro for escape (function)
macro_rules! Depcrateescape {
() => {
// Module: crate
// Provides: {"escape"}
// Dependencies: {}
# [doc = " Appends `value` to `output`, performing HTML-escaping in the process."] pub fn escape (value : & str , output : & mut String) { let value_str = value ; let mut last_emitted = 0 ; for (i , ch) in value . bytes () . enumerate () { match ch as char { '<' | '>' | '&' | '\'' | '"' => { output . push_str (& value_str [last_emitted .. i]) ; let s = match ch as char { '>' => "&gt;" , '<' => "&lt;" , '&' => "&amp;" , '\'' => "&#39;" , '"' => "&quot;" , _ => unreachable ! () , } ; output . push_str (s) ; last_emitted = i + 1 ; } _ => { } } } if last_emitted < value_str . len () { output . push_str (& value_str [last_emitted ..]) ; } }
};
}
