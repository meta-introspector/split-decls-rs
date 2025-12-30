// Generated macro for scan_table_head (function)
macro_rules! Depcrate_scannersscan_table_head {
() => {
// Module: crate::scanners
// Provides: {"scan_table_head"}
// Dependencies: {}
pub (crate) fn scan_table_head (data : & [u8]) -> (usize , Vec < Alignment >) { let (mut i , spaces) = calc_indent (data , 4) ; if spaces > 3 || i == data . len () { return (0 , vec ! []) ; } let mut cols = vec ! [] ; let mut active_col = Alignment :: None ; let mut start_col = true ; let mut found_pipe = false ; let mut found_hyphen = false ; let mut found_hyphen_in_col = false ; if data [i] == b'|' { i += 1 ; found_pipe = true ; } for c in & data [i ..] { if let Some (n) = scan_eol (& data [i ..]) { i += n ; break ; } match * c { b' ' => () , b':' => { active_col = match (start_col , active_col) { (true , Alignment :: None) => Alignment :: Left , (false , Alignment :: Left) => Alignment :: Center , (false , Alignment :: None) => Alignment :: Right , _ => active_col , } ; start_col = false ; } b'-' => { start_col = false ; found_hyphen = true ; found_hyphen_in_col = true ; } b'|' => { start_col = true ; found_pipe = true ; cols . push (active_col) ; active_col = Alignment :: None ; if ! found_hyphen_in_col { return (0 , vec ! []) ; } found_hyphen_in_col = false ; } _ => { return (0 , vec ! []) ; } } i += 1 ; } if ! start_col { cols . push (active_col) ; } if ! found_pipe || ! found_hyphen { return (0 , vec ! []) ; } (i , cols) }
};
}
