// Generated macro for crate_version (macro)
macro_rules! Depcrate_macroscrate_version {
() => {
// Module: crate::macros
// Provides: {"crate_version"}
// Dependencies: {}
# [doc = " Allows you to pull the version from your Cargo.toml at compile time as"] # [doc = " `MAJOR.MINOR.PATCH_PKGVERSION_PRE`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use clap_builder as clap;"] # [doc = " # use clap::crate_version;"] # [doc = " # use clap::Command;"] # [doc = " let m = Command::new(\"cmd\")"] # [doc = "             .version(crate_version!())"] # [doc = "             .get_matches();"] # [doc = " ```"] # [cfg (feature = "cargo")] # [macro_export] macro_rules ! crate_version { () => { env ! ("CARGO_PKG_VERSION") } ; }
};
}
