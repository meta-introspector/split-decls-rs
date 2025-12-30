// Generated macro for Opts (struct)
macro_rules! Depcrate_cliOpts {
() => {
// Module: crate::cli
// Provides: {"Opts"}
// Dependencies: {}
# [doc = " A helper utility to work with insta snapshots."] # [derive (Parser , Debug)] # [command (bin_name = "cargo insta" , arg_required_else_help = true , next_line_help = true)] struct Opts { # [doc = " Coloring"] # [arg (long , global = true , value_name = "WHEN" , env = "CARGO_TERM_COLOR")] color : Option < ColorWhen > , # [command (subcommand)] command : Command , }
};
}
