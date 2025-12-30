// Generated macro for split_heading (function)
macro_rules! Depcratesplit_heading {
() => {
// Module: crate
// Provides: {"split_heading"}
// Dependencies: {}
fn split_heading (s : & str) -> (Option < & str > , Option < & str > , & str) { let mut start = 0 ; while start < s . len () && s [start ..] . starts_with (' ') { start += 1 ; } let mut end = start ; while end < s . len () && s [end ..] . starts_with (| ch : char | ch . is_ascii_uppercase ()) { end += 1 ; } if end - start >= 3 && (end == s . len () || s [end ..] . starts_with (':')) { let indent = (start > 0) . then_some (& s [.. start]) ; let heading = & s [start .. end] ; let rest = & s [end ..] ; (indent , Some (heading) , rest) } else { (None , None , s) } }
};
}
