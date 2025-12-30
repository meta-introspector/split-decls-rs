// Generated macro for remove_base64_suffix (function)
macro_rules! Depcrateremove_base64_suffix {
() => {
// Module: crate
// Provides: {"remove_base64_suffix"}
// Dependencies: {}
# [doc = " None: no base64 suffix"] # [allow (clippy :: skip_while_next)] fn remove_base64_suffix (s : & str) -> Option < & str > { let mut bytes = s . bytes () ; { let iter = bytes . by_ref () . filter (| & byte | ! matches ! (byte , b'\t' | b'\n' | b'\r')) ; let mut iter = iter . rev () ; require ! (iter . next () ? == b'4') ; require ! (iter . next () ? == b'6') ; require ! (iter . next () ?. eq_ignore_ascii_case (& b'e')) ; require ! (iter . next () ?. eq_ignore_ascii_case (& b's')) ; require ! (iter . next () ?. eq_ignore_ascii_case (& b'a')) ; require ! (iter . next () ?. eq_ignore_ascii_case (& b'b')) ; require ! (iter . skip_while (|& byte | byte == b' ') . next () ? == b';') ; } Some (& s [.. bytes . len ()]) }
};
}
