// Generated macro for append_separator_if_needed (function)
macro_rules! Depcrateappend_separator_if_needed {
() => {
// Module: crate
// Provides: {"append_separator_if_needed"}
// Dependencies: {}
fn append_separator_if_needed (string : & mut String , start_position : usize) { if string . len () > start_position { string . push ('&') } }
};
}
