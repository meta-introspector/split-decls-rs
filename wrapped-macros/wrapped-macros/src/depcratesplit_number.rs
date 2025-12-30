// Generated macro for split_number (function)
macro_rules! Depcratesplit_number {
() => {
// Module: crate
// Provides: {"split_number"}
// Dependencies: {}
# [doc = " given a string src that begins with a text decimal number, return the tail (characters after the number) and the value of the decimal number"] fn split_number (src : & str) -> (& str , usize) { let mut rval = 0 ; let mut cursor = 0 ; let chars = src . chars () ; for (i , ch) in chars . enumerate () { match ch . to_digit (10) { Some (val) => { rval = rval * 10 + val as usize ; cursor = i + 1 ; } None => break , } } (& src [cursor ..] , rval) }
};
}
