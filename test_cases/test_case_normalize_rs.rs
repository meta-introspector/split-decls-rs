// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_trait_selection/src/traits/query/normalize.rs
// Error: expected square brackets
// Problematic line: line 10

use rustc_infer::traits::PredicateObligations;
use rustc_macros::extension;
pub use rustc_middle::traits::query::NormalizationResult;
use rustc_middle::ty::{
    self, FallibleTypeFolder, Ty, TyCtxt, TypeFoldable, TypeSuperFoldable, TypeSuperVisitable,
    TypeVisitable, TypeVisitableExt, TypeVisitor, TypingMode,
};
