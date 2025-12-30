// Generated macro for LldMode (enum)
macro_rules! Depcrate_core_config_toml_rustLldMode {
() => {
// Module: crate::core::config::toml::rust
// Provides: {"LldMode"}
// Dependencies: {}
# [doc = " LLD in bootstrap works like this:"] # [doc = " - Self-contained lld: use `rust-lld` from the compiler's sysroot"] # [doc = " - External: use an external `lld` binary"] # [doc = ""] # [doc = " It is configured depending on the target:"] # [doc = " 1) Everything except MSVC"] # [doc = " - Self-contained: `-Clinker-flavor=gnu-lld-cc -Clink-self-contained=+linker`"] # [doc = " - External: `-Clinker-flavor=gnu-lld-cc`"] # [doc = " 2) MSVC"] # [doc = " - Self-contained: `-Clinker=<path to rust-lld>`"] # [doc = " - External: `-Clinker=lld`"] # [derive (Copy , Clone , Default , Debug , PartialEq)] pub enum LldMode { # [doc = " Do not use LLD"] # [default] Unused , # [doc = " Use `rust-lld` from the compiler's sysroot"] SelfContained , # [doc = " Use an externally provided `lld` binary."] # [doc = " Note that the linker name cannot be overridden, the binary has to be named `lld` and it has"] # [doc = " to be in $PATH."] External , }
};
}
