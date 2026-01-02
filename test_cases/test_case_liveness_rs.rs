// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_passes/src/liveness.rs
// Error: expected square brackets
// Problematic line: line 111


mod rwu_table;

rustc_index::newtype_index! {
    #[debug_format = "v({})"]
    pub struct Variable {}
}
