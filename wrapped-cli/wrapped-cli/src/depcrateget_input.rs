// Generated macro for get_input (function)
macro_rules! Depcrateget_input {
() => {
// Module: crate
// Provides: {"get_input"}
// Dependencies: {}
# [doc = " Reads input from file if given a path, otherwise stdin"] fn get_input (input : Option < & String >) -> io :: Result < String > { if let Some (input) = input { fs :: read_to_string (input) } else { let mut buffer = String :: new () ; io :: stdin () . read_to_string (& mut buffer) ? ; Ok (buffer) } }
};
}
