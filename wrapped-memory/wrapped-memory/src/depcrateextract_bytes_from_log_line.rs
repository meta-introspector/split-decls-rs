// Generated macro for extract_bytes_from_log_line (function)
macro_rules! Depcrateextract_bytes_from_log_line {
() => {
// Module: crate
// Provides: {"extract_bytes_from_log_line"}
// Dependencies: {}
fn extract_bytes_from_log_line (preamble : & str , text : & str) -> u64 { let start = preamble . len () ; let end = text . find ("bytes") . expect ("Unable to find the word \"bytes\" in the dhat output.") ; text . get (start .. end) . expect ("Unable to get a substring.") . trim () . replace (',' , "") . parse :: < u64 > () . expect ("Unable to parse the byte amount") }
};
}
