// Generated macro for find_subcommand_with_path (function)
macro_rules! Depcrate_aot_generator_utilsfind_subcommand_with_path {
() => {
// Module: crate::aot::generator::utils
// Provides: {"find_subcommand_with_path"}
// Dependencies: {}
# [doc = " Finds the subcommand [`clap::Command`] from the given [`clap::Command`] with the given path."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " **NOTE:** `path` should not contain the root `bin_name`."] # [doc = ""] # [doc = " </div>"] pub fn find_subcommand_with_path < 'cmd > (p : & 'cmd Command , path : Vec < & str >) -> & 'cmd Command { let mut cmd = p ; for sc in path { cmd = cmd . find_subcommand (sc) . unwrap () ; } cmd }
};
}
