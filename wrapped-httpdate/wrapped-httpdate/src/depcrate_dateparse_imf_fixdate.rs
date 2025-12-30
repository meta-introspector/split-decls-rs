// Generated macro for parse_imf_fixdate (function)
macro_rules! Depcrate_dateparse_imf_fixdate {
() => {
// Module: crate::date
// Provides: {"parse_imf_fixdate"}
// Dependencies: {}
fn parse_imf_fixdate (s : & [u8]) -> Result < HttpDate , Error > { if s . len () != 29 || & s [25 ..] != b" GMT" || s [16] != b' ' || s [19] != b':' || s [22] != b':' { return Err (Error (())) ; } Ok (HttpDate { sec : toint_2 (& s [23 .. 25]) ? , min : toint_2 (& s [20 .. 22]) ? , hour : toint_2 (& s [17 .. 19]) ? , day : toint_2 (& s [5 .. 7]) ? , mon : match & s [7 .. 12] { b" Jan " => 1 , b" Feb " => 2 , b" Mar " => 3 , b" Apr " => 4 , b" May " => 5 , b" Jun " => 6 , b" Jul " => 7 , b" Aug " => 8 , b" Sep " => 9 , b" Oct " => 10 , b" Nov " => 11 , b" Dec " => 12 , _ => return Err (Error (())) , } , year : toint_4 (& s [12 .. 16]) ? , wday : match & s [.. 5] { b"Mon, " => 1 , b"Tue, " => 2 , b"Wed, " => 3 , b"Thu, " => 4 , b"Fri, " => 5 , b"Sat, " => 6 , b"Sun, " => 7 , _ => return Err (Error (())) , } , }) }
};
}
