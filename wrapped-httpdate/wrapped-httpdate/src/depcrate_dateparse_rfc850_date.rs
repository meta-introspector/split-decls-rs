// Generated macro for parse_rfc850_date (function)
macro_rules! Depcrate_dateparse_rfc850_date {
() => {
// Module: crate::date
// Provides: {"parse_rfc850_date"}
// Dependencies: {}
fn parse_rfc850_date (s : & [u8]) -> Result < HttpDate , Error > { if s . len () < 23 { return Err (Error (())) ; } fn wday < 'a > (s : & 'a [u8] , wday : u8 , name : & 'static [u8]) -> Option < (u8 , & 'a [u8]) > { if & s [0 .. name . len ()] == name { return Some ((wday , & s [name . len () ..])) ; } None } let (wday , s) = wday (s , 1 , b"Monday, ") . or_else (| | wday (s , 2 , b"Tuesday, ")) . or_else (| | wday (s , 3 , b"Wednesday, ")) . or_else (| | wday (s , 4 , b"Thursday, ")) . or_else (| | wday (s , 5 , b"Friday, ")) . or_else (| | wday (s , 6 , b"Saturday, ")) . or_else (| | wday (s , 7 , b"Sunday, ")) . ok_or (Error (())) ? ; if s . len () != 22 || s [12] != b':' || s [15] != b':' || & s [18 .. 22] != b" GMT" { return Err (Error (())) ; } let mut year = u16 :: from (toint_2 (& s [7 .. 9]) ?) ; if year < 70 { year += 2000 ; } else { year += 1900 ; } Ok (HttpDate { sec : toint_2 (& s [16 .. 18]) ? , min : toint_2 (& s [13 .. 15]) ? , hour : toint_2 (& s [10 .. 12]) ? , day : toint_2 (& s [0 .. 2]) ? , mon : match & s [2 .. 7] { b"-Jan-" => 1 , b"-Feb-" => 2 , b"-Mar-" => 3 , b"-Apr-" => 4 , b"-May-" => 5 , b"-Jun-" => 6 , b"-Jul-" => 7 , b"-Aug-" => 8 , b"-Sep-" => 9 , b"-Oct-" => 10 , b"-Nov-" => 11 , b"-Dec-" => 12 , _ => return Err (Error (())) , } , year , wday , }) }
};
}
