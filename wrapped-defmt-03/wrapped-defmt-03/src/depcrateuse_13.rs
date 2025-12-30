// Generated macro for use_13 (pub_use)
macro_rules! Depcrateuse_13 {
() => {
// Module: crate
// Provides: {"use_13"}
// Dependencies: {}
# [doc = " Overrides the panicking behavior of `defmt::panic!`"] # [doc = ""] # [doc = " By default, `defmt::panic!` calls `core::panic!` after logging the panic message using `defmt`."] # [doc = " This can result in the panic message being printed twice in some cases. To avoid that issue use"] # [doc = " this macro. See [the manual] for details."] # [doc = ""] # [doc = " [the manual]: https://defmt.ferrous-systems.com/panic.html"] # [doc = ""] # [doc = " # Inter-operation with built-in attributes"] # [doc = ""] # [doc = " This attribute cannot be used together with the `export_name` or `no_mangle` attributes"] pub use defmt10 :: panic_handler ;
};
}
