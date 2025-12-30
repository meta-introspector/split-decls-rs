// Generated macro for thousands_sep (function)
macro_rules! Depcrate_formatthousands_sep {
() => {
// Module: crate::format
// Provides: {"thousands_sep"}
// Dependencies: {}
# [doc = " Format a number with thousands separators."] fn thousands_sep (mut n : u64 , sep : char) -> String { use std :: fmt :: Write ; let mut output = String :: new () ; let mut trailing = false ; for & pow in & [9 , 6 , 3 , 0] { let base = 10_u64 . pow (pow) ; if pow == 0 || trailing || n / base != 0 { if ! trailing { write ! (output , "{}" , n / base) . unwrap () ; } else { write ! (output , "{:03}" , n / base) . unwrap () ; } if pow != 0 { output . push (sep) ; } trailing = true ; } n %= base ; } output }
};
}
