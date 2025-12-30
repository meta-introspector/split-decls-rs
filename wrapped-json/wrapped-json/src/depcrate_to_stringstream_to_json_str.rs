// Generated macro for stream_to_json_str (function)
macro_rules! Depcrate_to_stringstream_to_json_str {
() => {
// Module: crate::to_string
// Provides: {"stream_to_json_str"}
// Dependencies: {}
# [doc = "\nStream a value as JSON into a `JsonStr`.\n\nThis method will fail if the value contains complex values as keys.\n"] pub fn stream_to_json_str (v : impl sval :: Value) -> Result < Box < JsonStr > , Error > { Ok (JsonStr :: boxed (stream_to_string (v) ?)) }
};
}
