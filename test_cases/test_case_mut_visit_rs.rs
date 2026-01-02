// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_ast/src/mut_visit.rs
// Error: expected square brackets
// Problematic line: line 23

use crate::tokenstream::*;
use crate::visit::{AssocCtxt, BoundKind, FnCtxt, LifetimeCtxt, VisitorResult, try_visit};

mod sealed {
    use rustc_ast_ir::visit::VisitorResult;

    /// This is for compatibility with the regular `Visitor`.
