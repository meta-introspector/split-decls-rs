// Generated macro for all_subcommands (function)
macro_rules! Depcrate_aot_generator_utilsall_subcommands {
() => {
// Module: crate::aot::generator::utils
// Provides: {"all_subcommands"}
// Dependencies: {}
# [doc = " Gets all subcommands including child subcommands in the form of `(\"name\", \"bin_name\")`."] # [doc = ""] # [doc = " Subcommand `rustup toolchain install` would be converted to"] # [doc = " `(\"install\", \"rustup toolchain install\")`."] pub fn all_subcommands (cmd : & Command) -> Vec < (String , String) > { let mut subcmds : Vec < _ > = subcommands (cmd) ; for sc_v in cmd . get_subcommands () . map (all_subcommands) { subcmds . extend (sc_v) ; } subcmds }
};
}
