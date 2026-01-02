// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir/src/intravisit.rs
// Error: expected square brackets
// Problematic line: line 74


use crate::hir::*;

pub trait IntoVisitor<'hir> {
    type Visitor: Visitor<'hir>;
    fn into_visitor(&self) -> Self::Visitor;
}
