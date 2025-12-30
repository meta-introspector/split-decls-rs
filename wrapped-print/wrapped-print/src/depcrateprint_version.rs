// Generated macro for print_version (function)
macro_rules! Depcrateprint_version {
() => {
// Module: crate
// Provides: {"print_version"}
// Dependencies: {}
# [doc = " Report version from Cargo.toml _(e.g. \"0.1.4\")_ and supported `defmt`-versions."] # [doc = ""] # [doc = " Used by `--version` flag."] # [allow (clippy :: unnecessary_wraps)] fn print_version () -> anyhow :: Result < () > { println ! ("{} {}" , env ! ("CARGO_PKG_NAME") , env ! ("CARGO_PKG_VERSION")) ; let s = if DEFMT_VERSIONS . len () > 1 { "s" } else { "" } ; println ! ("supported defmt version{s}: {}" , DEFMT_VERSIONS . join (", ")) ; Ok (()) }
};
}
