// Generated macro for odb (module)
macro_rules! Depcrate_plumbing_optionsodb {
() => {
// Module: crate::plumbing::options
// Provides: {"odb"}
// Dependencies: {}
pub mod odb { # [derive (Debug , clap :: Subcommand)] pub enum Subcommands { # [doc = " Print all object names."] Entries , # [doc = " Provide general information about the object database."] Info , # [doc = " Count and obtain information on all, possibly duplicate, objects in the database."] # [clap (visible_alias = "statistics")] Stats { # [doc = " Lookup headers again, but without preloading indices."] # [clap (long)] extra_header_lookup : bool , } , } }
};
}
