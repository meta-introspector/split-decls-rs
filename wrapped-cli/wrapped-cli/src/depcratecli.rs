// Generated macro for Cli (struct)
macro_rules! DepcrateCli {
() => {
// Module: crate
// Provides: {"Cli"}
// Dependencies: {}
# [derive (Parser)] # [command (version , about)] struct Cli { # [command (subcommand)] command : Option < Commands > , }
};
}
