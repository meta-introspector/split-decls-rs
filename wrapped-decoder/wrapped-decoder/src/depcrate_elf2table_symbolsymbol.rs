// Generated macro for Symbol (struct)
macro_rules! Depcrate_elf2table_symbolSymbol {
() => {
// Module: crate::elf2table::symbol
// Provides: {"Symbol"}
// Dependencies: {}
# [derive (Deserialize , PartialEq , Eq , Hash)] pub (super) struct Symbol { # [doc = " Name of the Cargo package in which the symbol is being instantiated. Used for avoiding"] # [doc = " symbol name collisions."] package : String , # [doc = " Unique identifier that disambiguates otherwise equivalent invocations in the same crate."] disambiguator : String , # [doc = " Symbol categorization. Known values:"] # [doc = " * `defmt_prim` for primitive formatting strings that are placed at the start of the `.defmt`"] # [doc = "   section."] # [doc = " * `defmt_fmt`, `defmt_str` for interned format strings and string literals."] # [doc = " * `defmt_trace`, `defmt_debug`, `defmt_info`, `defmt_warn`, `defmt_error` for logging"] # [doc = "   messages used at the different log levels."] # [doc = " * Anything starting with `defmt_` is reserved for use by defmt, other prefixes are free for"] # [doc = "   use by third-party apps (but they all should use a prefix!)."] tag : String , # [doc = " Symbol data for use by the host tooling. Interpretation depends on `tag`."] data : String , # [doc = " Crate name obtained via CARGO_CRATE_NAME (added since a Cargo package can contain many crates)."] crate_name : Option < String > , }
};
}
