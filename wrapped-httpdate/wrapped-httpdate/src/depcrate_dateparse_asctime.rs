// Generated macro for parse_asctime (function)
macro_rules! Depcrate_dateparse_asctime {
() => {
// Module: crate::date
// Provides: {"parse_asctime"}
// Dependencies: {}
fn parse_asctime (s : & [u8]) -> Result < HttpDate , Error > { if s . len () != 24 || s [10] != b' ' || s [13] != b':' || s [16] != b':' || s [19] != b' ' { return Err (Error (())) ; } Ok (HttpDate { sec : toint_2 (& s [17 .. 19]) ? , min : toint_2 (& s [14 .. 16]) ? , hour : toint_2 (& s [11 .. 13]) ? , day : { let x = & s [8 .. 10] ; { if x [0] == b' ' { toint_1 (x [1]) } else { toint_2 (x) } } ? } , mon : match & s [4 .. 8] { b"Jan " => 1 , b"Feb " => 2 , b"Mar " => 3 , b"Apr " => 4 , b"May " => 5 , b"Jun " => 6 , b"Jul " => 7 , b"Aug " => 8 , b"Sep " => 9 , b"Oct " => 10 , b"Nov " => 11 , b"Dec " => 12 , _ => return Err (Error (())) , } , year : toint_4 (& s [20 .. 24]) ? , wday : match & s [0 .. 4] { b"Mon " => 1 , b"Tue " => 2 , b"Wed " => 3 , b"Thu " => 4 , b"Fri " => 5 , b"Sat " => 6 , b"Sun " => 7 , _ => return Err (Error (())) , } , }) }
};
}
