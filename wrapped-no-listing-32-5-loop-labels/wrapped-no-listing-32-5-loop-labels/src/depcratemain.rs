// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let mut count = 0 ; 'counting_up : loop { println ! ("count = {count}") ; let mut remaining = 10 ; loop { println ! ("remaining = {remaining}") ; if remaining == 9 { break ; } if count == 2 { break 'counting_up ; } remaining -= 1 ; } count += 1 ; } println ! ("End count = {count}") ; }
};
}
