// Generated macro for DiscoverCommand (struct)
macro_rules! Depcrate_discoverDiscoverCommand {
() => {
// Module: crate::discover
// Provides: {"DiscoverCommand"}
// Dependencies: {}
# [doc = " A command wrapper for getting a `rust-project.json`."] # [doc = ""] # [doc = " This is analogous to discovering a cargo project + running `cargo-metadata` on it, but for non-Cargo build systems."] pub (crate) struct DiscoverCommand { command : Vec < String > , sender : Sender < DiscoverProjectMessage > , }
};
}
