// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { println ! ("Searching for Rust compilers in /nix/store...") ; let all_rustc_paths : Vec < String > = find_nix_rustc ! () ; println ! ("\nFound ALL Rust compilers:") ; for path in all_rustc_paths { println ! ("  {}\n" , path) ; } let rustc_191_paths : Vec < String > = find_nix_rustc ! ("1.91") ; println ! ("\nFound Rust compilers containing '1.91':") ; for path in rustc_191_paths { println ! ("  {}\n" , path) ; } println ! ("\nSearch complete.") ; }
};
}
