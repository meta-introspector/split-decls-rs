// Generated macro for parse_range (function)
macro_rules! Depcrateparse_range {
() => {
// Module: crate
// Provides: {"parse_range"}
// Dependencies: {}
fn parse_range (mut s : & str) -> Option < (Range < u8 > , usize) > { let start_digits = s . as_bytes () . iter () . take_while (| b | (* * b as char) . is_ascii_digit ()) . count () ; let start = s [.. start_digits] . parse () . ok () ? ; if & s [start_digits .. start_digits + 2] != ".." { return None ; } s = & s [start_digits + 2 ..] ; let end_digits = s . as_bytes () . iter () . take_while (| b | (* * b as char) . is_ascii_digit ()) . count () ; let end = s [.. end_digits] . parse () . ok () ? ; if end <= start || start >= 128 || end > 128 { return None ; } Some ((start .. end , start_digits + end_digits + 2)) }
};
}
