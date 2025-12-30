// Generated macro for read_file (function)
macro_rules! Depcrateread_file {
() => {
// Module: crate
// Provides: {"read_file"}
// Dependencies: {}
fn read_file < P : AsRef < Path > > (path : P) -> io :: Result < String > { let mut file = File :: open (path . as_ref ()) ? ; let mut string = String :: new () ; file . read_to_string (& mut string) ? ; Ok (string) }
};
}
