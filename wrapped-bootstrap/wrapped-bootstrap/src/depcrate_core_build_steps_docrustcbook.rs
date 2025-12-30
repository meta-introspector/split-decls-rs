// Generated macro for RustcBook (struct)
macro_rules! Depcrate_core_build_steps_docRustcBook {
() => {
// Module: crate::core::build_steps::doc
// Provides: {"RustcBook"}
// Dependencies: {}
# [doc = " Builds the Rust compiler book."] # [derive (Debug , Clone , Hash , PartialEq , Eq)] pub struct RustcBook { build_compiler : Compiler , target : TargetSelection , # [doc = " Test that the examples of lints in the book produce the correct lints in the expected"] # [doc = " format."] validate : bool , }
};
}
