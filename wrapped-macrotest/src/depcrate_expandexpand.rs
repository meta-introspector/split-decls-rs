// Generated macro for expand (function)
macro_rules! Depcrate_expandexpand {
() => {
// Module: crate::expand
// Provides: {"expand"}
// Dependencies: {}
# [doc = " Attempts to expand macros in files that match glob pattern."] # [doc = ""] # [doc = " # Refresh behavior"] # [doc = ""] # [doc = " If no matching `.expanded.rs` files present, they will be created and result of expansion"] # [doc = " will be written into them."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Will panic if matching `.expanded.rs` file is present, but has different expanded code in it."] pub fn expand (path : impl AsRef < Path >) { run_tests (path , ExpansionBehavior :: RegenerateFiles , Option :: < Vec < String > > :: None ,) ; }
};
}
