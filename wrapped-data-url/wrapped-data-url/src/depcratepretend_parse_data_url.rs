// Generated macro for pretend_parse_data_url (function)
macro_rules! Depcratepretend_parse_data_url {
() => {
// Module: crate
// Provides: {"pretend_parse_data_url"}
// Dependencies: {}
# [doc = " Similar to <https://url.spec.whatwg.org/#concept-basic-url-parser>"] # [doc = " followed by <https://url.spec.whatwg.org/#concept-url-serializer>"] # [doc = ""] # [doc = " * `None`: not a data URL."] # [doc = ""] # [doc = " * `Some(s)`: sort of the result of serialization, except:"] # [doc = ""] # [doc = "   - `data:` prefix removed"] # [doc = "   - The fragment is included"] # [doc = "   - Other components are **not** UTF-8 percent-encoded"] # [doc = "   - ASCII tabs and newlines in the middle are **not** removed"] fn pretend_parse_data_url (input : & str) -> Option < & str > { let left_trimmed = input . trim_start_matches (| ch | ch <= ' ') ; let mut bytes = left_trimmed . bytes () ; { let mut iter = bytes . by_ref () . filter (| & byte | ! matches ! (byte , b'\t' | b'\n' | b'\r')) ; require ! (iter . next () ?. eq_ignore_ascii_case (& b'd')) ; require ! (iter . next () ?. eq_ignore_ascii_case (& b'a')) ; require ! (iter . next () ?. eq_ignore_ascii_case (& b't')) ; require ! (iter . next () ?. eq_ignore_ascii_case (& b'a')) ; require ! (iter . next () ? == b':') ; } let bytes_consumed = left_trimmed . len () - bytes . len () ; let after_colon = & left_trimmed [bytes_consumed ..] ; Some (after_colon . trim_end_matches (| ch | ch <= ' ')) }
};
}
