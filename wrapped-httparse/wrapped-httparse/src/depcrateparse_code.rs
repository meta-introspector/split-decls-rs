// Generated macro for parse_code (function)
macro_rules! Depcrateparse_code {
() => {
// Module: crate
// Provides: {"parse_code"}
// Dependencies: {}
# [inline] fn parse_code (bytes : & mut Bytes < '_ >) -> Result < u16 > { let hundreds = expect ! (bytes . next () == b'0' ..= b'9' => Err (Error :: Status)) ; let tens = expect ! (bytes . next () == b'0' ..= b'9' => Err (Error :: Status)) ; let ones = expect ! (bytes . next () == b'0' ..= b'9' => Err (Error :: Status)) ; Ok (Status :: Complete ((hundreds - b'0') as u16 * 100 + (tens - b'0') as u16 * 10 + (ones - b'0') as u16)) }
};
}
