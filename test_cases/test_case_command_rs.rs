// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_ssa/src/back/command.rs
// Error: expected square brackets
// Problematic line: line 10


use rustc_target::spec::LldFlavor;

#[derive(Clone)]
pub(crate) struct Command {
    program: Program,
    args: Vec<OsString>,
