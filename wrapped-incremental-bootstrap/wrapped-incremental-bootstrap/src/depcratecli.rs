// Generated macro for Cli (struct)
macro_rules! DepcrateCli {
() => {
// Module: crate
// Provides: {"Cli"}
// Dependencies: {}
# [derive (Parser , Debug)] # [command (author , version , about = "Incremental Bootstrap Tool")] struct Cli { # [command (subcommand)] command : Commands , }
};
}
