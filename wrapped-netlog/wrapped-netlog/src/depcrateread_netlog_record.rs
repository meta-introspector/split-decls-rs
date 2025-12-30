// Generated macro for read_netlog_record (function)
macro_rules! Depcrateread_netlog_record {
() => {
// Module: crate
// Provides: {"read_netlog_record"}
// Dependencies: {}
# [doc = " Reads a single record from a netlog file accessed by a BufRead."] pub fn read_netlog_record < R : BufRead > (reader : & mut R) -> Option < Vec < u8 > > { let mut buf = Vec :: < u8 > :: new () ; let size = reader . read_until (b'\n' , & mut buf) . unwrap () ; if size <= 1 { return None ; } if buf [0] != b'{' { return None ; } buf . truncate (buf . len () - 2) ; if buf [buf . len () - 1] == b']' { buf . truncate (buf . len () - 1) ; } log :: trace ! ("read record={}" , String :: from_utf8 (buf . clone ()) . expect ("from_utf8 failed")) ; Some (buf) }
};
}
