// Generated macro for unwrap (function)
macro_rules! Depcrate_utilunwrap {
() => {
// Module: crate::util
// Provides: {"unwrap"}
// Dependencies: {}
# [doc = " Removes tags from the front and back of a string."] pub fn unwrap < 't > (text : & 't str , front : & str , back : & str) -> & 't str { text . trim () . trim_start_matches (front) . trim_end_matches (back) }
};
}
