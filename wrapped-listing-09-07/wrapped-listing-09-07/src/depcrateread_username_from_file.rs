// Generated macro for read_username_from_file (function)
macro_rules! Depcrateread_username_from_file {
() => {
// Module: crate
// Provides: {"read_username_from_file"}
// Dependencies: {}
fn read_username_from_file () -> Result < String , io :: Error > { let mut username_file = File :: open ("hello.txt") ? ; let mut username = String :: new () ; username_file . read_to_string (& mut username) ? ; Ok (username) }
};
}
