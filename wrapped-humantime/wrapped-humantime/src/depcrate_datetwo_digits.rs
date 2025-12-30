// Generated macro for two_digits (function)
macro_rules! Depcrate_datetwo_digits {
() => {
// Module: crate::date
// Provides: {"two_digits"}
// Dependencies: {}
# [inline] # [doc = " Converts two digits given in ASCII to its proper decimal representation."] fn two_digits (b1 : u8 , b2 : u8) -> Result < u64 , Error > { fn two_digits_inner (a : char , b : char) -> Option < u64 > { let a = a . to_digit (10) ? ; let b = b . to_digit (10) ? ; Some ((a * 10 + b) as u64) } two_digits_inner (b1 as char , b2 as char) . ok_or (Error :: InvalidDigit) }
};
}
