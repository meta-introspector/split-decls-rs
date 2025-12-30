// Generated macro for read_file (function)
macro_rules! Depcrateread_file {
() => {
// Module: crate
// Provides: {"read_file"}
// Dependencies: {}
fn read_file (path : & str) -> Result < String , io :: Error > { let mut f = File :: open (path) ? ; let mut s = String :: new () ; f . read_to_string (& mut s) ? ; Ok (s) }
};
}
