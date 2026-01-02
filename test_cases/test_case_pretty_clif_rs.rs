// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/pretty_clif.rs
// Error: expected square brackets
// Problematic line: line 72


use crate::prelude::*;

#[derive(Clone, Debug)]
pub(crate) struct CommentWriter {
    enabled: bool,
    global_comments: Vec<String>,
