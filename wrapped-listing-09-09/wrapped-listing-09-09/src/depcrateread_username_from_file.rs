// Generated macro for read_username_from_file (function)
macro_rules! Depcrateread_username_from_file {
() => {
// Module: crate
// Provides: {"read_username_from_file"}
// Dependencies: {}
fn read_username_from_file () -> Result < String , io :: Error > { fs :: read_to_string ("hello.txt") }
};
}
