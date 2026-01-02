// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_infer/src/infer/canonical/instantiate.rs
// Error: expected square brackets
// Problematic line: line 10

//! [c]: https://rust-lang.github.io/chalk/book/canonical_queries/canonicalization.html

use rustc_macros::extension;
use rustc_middle::ty::{
    self, DelayedMap, Ty, TyCtxt, TypeFoldable, TypeFolder, TypeSuperFoldable, TypeSuperVisitable,
    TypeVisitableExt, TypeVisitor,
};
