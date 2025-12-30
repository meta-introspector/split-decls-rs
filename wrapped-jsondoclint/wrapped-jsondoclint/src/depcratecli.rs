// Generated macro for Cli (struct)
macro_rules! DepcrateCli {
() => {
// Module: crate
// Provides: {"Cli"}
// Dependencies: {}
# [derive (Parser)] struct Cli { # [doc = " The path to the json file to be linted"] path : String , # [doc = " Show verbose output"] # [arg (long)] verbose : bool , # [arg (long)] json_output : Option < String > , }
};
}
