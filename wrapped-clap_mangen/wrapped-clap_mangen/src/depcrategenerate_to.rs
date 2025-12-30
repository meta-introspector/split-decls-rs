// Generated macro for generate_to (function)
macro_rules! Depcrategenerate_to {
() => {
// Module: crate
// Provides: {"generate_to"}
// Dependencies: {}
# [doc = " Generate manual page files for the command with all subcommands"] pub fn generate_to (cmd : clap :: Command , out_dir : impl AsRef < std :: path :: Path > ,) -> Result < () , std :: io :: Error > { fn generate (cmd : clap :: Command , out_dir : & std :: path :: Path) -> Result < () , std :: io :: Error > { for cmd in cmd . get_subcommands () . filter (| s | ! s . is_hide_set ()) . cloned () { generate (cmd , out_dir) ? ; } Man :: new (cmd) . generate_to (out_dir) ? ; Ok (()) } let mut cmd = cmd . disable_help_subcommand (true) ; cmd . build () ; generate (cmd , out_dir . as_ref ()) }
};
}
