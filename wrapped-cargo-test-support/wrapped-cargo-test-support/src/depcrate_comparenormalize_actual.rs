// Generated macro for normalize_actual (function)
macro_rules! Depcrate_comparenormalize_actual {
() => {
// Module: crate::compare
// Provides: {"normalize_actual"}
// Dependencies: {}
# [doc = " Normalizes the output so that it can be compared against the expected value."] fn normalize_actual (content : & str , redactions : & snapbox :: Redactions) -> String { use snapbox :: filter :: Filter as _ ; let content = snapbox :: filter :: FilterPaths . filter (content . into_data ()) ; let content = snapbox :: filter :: FilterNewlines . filter (content) ; let content = content . render () . expect ("came in as a String") ; let content = redactions . redact (& content) ; content }
};
}
