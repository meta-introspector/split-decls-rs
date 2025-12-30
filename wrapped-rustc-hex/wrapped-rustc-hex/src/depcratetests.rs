// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { FromHex , ToHex } ; # [cfg (not (feature = "std"))] extern crate alloc ; # [cfg (not (feature = "std"))] use alloc :: { string :: String , vec :: Vec , format } ; # [test] pub fn test_to_hex () { assert_eq ! ("foobar" . as_bytes () . to_hex ::< String > () , "666f6f626172") ; } # [test] pub fn test_from_hex_okay () { assert_eq ! ("666f6f626172" . from_hex ::< Vec < _ >> () . unwrap () , b"foobar") ; assert_eq ! ("666F6F626172" . from_hex ::< Vec < _ >> () . unwrap () , b"foobar") ; } # [test] pub fn test_from_hex_odd_len () { assert ! ("666" . from_hex ::< Vec < _ >> () . is_err ()) ; assert ! ("66 6" . from_hex ::< Vec < _ >> () . is_err ()) ; } # [test] pub fn test_from_hex_invalid_char () { assert ! ("66y6" . from_hex ::< Vec < _ >> () . is_err ()) ; } # [test] pub fn test_from_hex_ignores_whitespace () { assert_eq ! ("666f 6f6\r\n26172 " . from_hex ::< Vec < _ >> () . unwrap () , b"foobar") ; } # [test] pub fn test_to_hex_all_bytes () { for i in 0 .. 256 { assert_eq ! ([i as u8] . to_hex ::< String > () , format ! ("{:02x}" , i)) ; } } # [test] pub fn test_from_hex_all_bytes () { for i in 0 .. 256 { let ii : & [u8] = & [i as u8] ; assert_eq ! (format ! ("{:02x}" , i) . from_hex ::< Vec < _ >> () . unwrap () , ii) ; assert_eq ! (format ! ("{:02X}" , i) . from_hex ::< Vec < _ >> () . unwrap () , ii) ; } } }
};
}
