// Generated macro for crate_authors (macro)
macro_rules! Depcrate_macroscrate_authors {
() => {
// Module: crate::macros
// Provides: {"crate_authors"}
// Dependencies: {}
# [doc = " Allows you to pull the authors for the command from your Cargo.toml at"] # [doc = " compile time in the form:"] # [doc = " `\"author1 lastname <author1@example.com>:author2 lastname <author2@example.com>\"`"] # [doc = ""] # [doc = " You can replace the colons with a custom separator by supplying a"] # [doc = " replacement string, so, for example,"] # [doc = " `crate_authors!(\",\\n\")` would become"] # [doc = " `\"author1 lastname <author1@example.com>,\\nauthor2 lastname <author2@example.com>,\\nauthor3 lastname <author3@example.com>\"`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use clap_builder as clap;"] # [doc = " # use clap::crate_authors;"] # [doc = " # use clap::Command;"] # [doc = " let m = Command::new(\"cmd\")"] # [doc = "             .author(crate_authors!(\"\\n\"))"] # [doc = "             .get_matches();"] # [doc = " ```"] # [cfg (feature = "cargo")] # [macro_export] macro_rules ! crate_authors { ($ sep : expr) => { { static AUTHORS : & str = env ! ("CARGO_PKG_AUTHORS") ; if AUTHORS . contains (':') { static CACHED : std :: sync :: OnceLock < String > = std :: sync :: OnceLock :: new () ; let s = CACHED . get_or_init (|| AUTHORS . replace (':' , $ sep)) ; let s : &'static str = &* s ; s } else { AUTHORS } } } ; () => { env ! ("CARGO_PKG_AUTHORS") } ; }
};
}
