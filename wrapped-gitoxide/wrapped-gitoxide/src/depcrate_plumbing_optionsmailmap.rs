// Generated macro for mailmap (module)
macro_rules! Depcrate_plumbing_optionsmailmap {
() => {
// Module: crate::plumbing::options
// Provides: {"mailmap"}
// Dependencies: {}
pub mod mailmap { use gix :: bstr :: BString ; # [derive (Debug , clap :: Subcommand)] pub enum Subcommands { # [doc = " Print all entries in configured mailmaps, inform about errors as well."] Entries , # [doc = " Print the canonical form of contacts according to the configured mailmaps."] Check { # [doc = " One or more `Name <email>` or `<email>` to pass through the mailmap."] contacts : Vec < BString > , } , } }
};
}
