// Generated macro for mailmap (module)
macro_rules! Depcrate_plumbing_options_freemailmap {
() => {
// Module: crate::plumbing::options::free
// Provides: {"mailmap"}
// Dependencies: {}
# [doc = ""] pub mod mailmap { use std :: path :: PathBuf ; # [derive (Debug , clap :: Parser)] pub struct Platform { # [doc = " The path to the mailmap file."] # [clap (short = 'p' , long , default_value = ".mailmap")] pub path : PathBuf , # [doc = " Subcommands"] # [clap (subcommand)] pub cmd : Subcommands , } # [derive (Debug , clap :: Subcommand)] pub enum Subcommands { # [doc = " Parse all entries in the mailmap and report malformed lines or collisions."] Verify , } }
};
}
