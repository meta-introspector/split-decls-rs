// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { use std :: collections :: HashMap ; let text = "hello world wonderful world" ; let mut map = HashMap :: new () ; for word in text . split_whitespace () { let count = map . entry (word) . or_insert (0) ; * count += 1 ; } println ! ("{map:?}") ; }
};
}
