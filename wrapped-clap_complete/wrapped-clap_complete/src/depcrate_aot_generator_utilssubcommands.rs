// Generated macro for subcommands (function)
macro_rules! Depcrate_aot_generator_utilssubcommands {
() => {
// Module: crate::aot::generator::utils
// Provides: {"subcommands"}
// Dependencies: {}
# [doc = " Gets subcommands of [`clap::Command`] in the form of `(\"name\", \"bin_name\")`."] # [doc = ""] # [doc = " Subcommand `rustup toolchain install` would be converted to"] # [doc = " `(\"install\", \"rustup toolchain install\")`."] pub fn subcommands (p : & Command) -> Vec < (String , String) > { debug ! ("subcommands: name={}" , p . get_name ()) ; debug ! ("subcommands: Has subcommands...{:?}" , p . has_subcommands ()) ; let mut subcmds = vec ! [] ; for sc in p . get_subcommands () { let sc_bin_name = sc . get_bin_name () . unwrap () ; debug ! ("subcommands:iter: name={}, bin_name={}" , sc . get_name () , sc_bin_name) ; subcmds . push ((sc . get_name () . to_string () , sc_bin_name . to_string ())) ; for alias in sc . get_visible_aliases () { debug ! ("subcommands:iter: alias={}, bin_name={}" , alias , sc_bin_name) ; subcmds . push ((alias . to_string () , sc_bin_name . to_string ())) ; } } subcmds }
};
}
