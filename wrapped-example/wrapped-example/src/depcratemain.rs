// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
# [entry] fn main () -> ! { let x = 42 ; loop { asm :: nop () ; let mut hstdout = semihosting :: hio :: hstdout () . unwrap () ; let _ = write ! (hstdout , "x = {}\n" , x) ; semihosting :: debug :: exit (semihosting :: debug :: EXIT_SUCCESS) ; } }
};
}
