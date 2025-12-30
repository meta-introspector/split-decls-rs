// Generated macro for get_line_num (function)
macro_rules! Depcrateget_line_num {
() => {
// Module: crate
// Provides: {"get_line_num"}
// Dependencies: {}
fn get_line_num (source : & str , pos : usize) -> usize { let mut ptr = 0 ; let mut i = 0 ; let lines = source . lines () ; for line in lines { let lnlen = line . chars () . count () ; ptr += lnlen + 1 ; if ptr > pos { break ; } i += 1 ; } i }
};
}
