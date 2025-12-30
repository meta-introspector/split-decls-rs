// Generated macro for collect_dates_from_file (function)
macro_rules! Depcratecollect_dates_from_file {
() => {
// Module: crate
// Provides: {"collect_dates_from_file"}
// Dependencies: {}
fn collect_dates_from_file (date_regex : & Regex , text : & str) -> Vec < (usize , Date) > { let mut line = 1 ; let mut end_of_last_cap = 0 ; date_regex . captures_iter (text) . filter_map (| cap | { if let (Some (month) , Some (year) , None , None) | (None , None , Some (month) , Some (year)) = (cap . name ("m1") , cap . name ("y1") , cap . name ("m2") , cap . name ("y2")) { let year = year . as_str () . parse () . expect ("year") ; let month = Month :: from_str (month . as_str ()) . expect ("month") . number_from_month () ; Some ((cap . get (0) . expect ("all") . range () , Date { year , month })) } else { None } }) . map (| (byte_range , date) | { line += text [end_of_last_cap .. byte_range . end] . chars () . filter (| c | * c == '\n') . count () ; end_of_last_cap = byte_range . end ; (line , date) }) . collect () }
};
}
