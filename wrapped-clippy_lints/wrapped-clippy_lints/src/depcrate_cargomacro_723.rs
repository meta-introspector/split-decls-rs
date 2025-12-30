// Generated macro for macro_723 (macro)
macro_rules! Depcrate_cargomacro_723 {
() => {
// Module: crate::cargo
// Provides: {"macro_723"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for wildcard dependencies in the `Cargo.toml`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " [As the edition guide says](https://rust-lang-nursery.github.io/edition-guide/rust-2018/cargo-and-crates-io/crates-io-disallows-wildcard-dependencies.html),"] # [doc = " it is highly unlikely that you work with any possible version of your dependency,"] # [doc = " and wildcard dependencies would cause unnecessary breakage in the ecosystem."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```toml"] # [doc = " [dependencies]"] # [doc = " regex = \"*\""] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```toml"] # [doc = " [dependencies]"] # [doc = " # allow patch updates, but not minor or major version changes"] # [doc = " some_crate_1 = \"~1.2.3\""] # [doc = ""] # [doc = " # pin the version to a specific version"] # [doc = " some_crate_2 = \"=1.2.3\""] # [doc = " ```"] # [clippy :: version = "1.32.0"] pub WILDCARD_DEPENDENCIES , cargo , "wildcard dependencies being used" }
};
}
