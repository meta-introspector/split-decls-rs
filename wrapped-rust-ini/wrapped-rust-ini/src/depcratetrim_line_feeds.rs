// Generated macro for trim_line_feeds (function)
macro_rules! Depcratetrim_line_feeds {
() => {
// Module: crate
// Provides: {"trim_line_feeds"}
// Dependencies: {}
fn trim_line_feeds (string : & mut String) { const LF : char = '\n' ; string . truncate (string . trim_end_matches (LF) . len ()) ; string . drain (.. (string . len () - string . trim_start_matches (LF) . len ())) ; }
};
}
