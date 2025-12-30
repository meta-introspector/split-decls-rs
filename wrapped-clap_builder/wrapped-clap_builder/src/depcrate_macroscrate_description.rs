// Generated macro for crate_description (macro)
macro_rules! Depcrate_macroscrate_description {
() => {
// Module: crate::macros
// Provides: {"crate_description"}
// Dependencies: {}
# [doc = " Allows you to pull the description from your Cargo.toml at compile time."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use clap_builder as clap;"] # [doc = " # use clap::crate_description;"] # [doc = " # use clap::Command;"] # [doc = " let m = Command::new(\"cmd\")"] # [doc = "             .about(crate_description!())"] # [doc = "             .get_matches();"] # [doc = " ```"] # [cfg (feature = "cargo")] # [macro_export] macro_rules ! crate_description { () => { env ! ("CARGO_PKG_DESCRIPTION") } ; }
};
}
