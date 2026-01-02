// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_trait_selection/src/traits/wf.rs
// Error: expected square brackets
// Problematic line: line 13

use rustc_hir::lang_items::LangItem;
use rustc_infer::traits::{ObligationCauseCode, PredicateObligations};
use rustc_middle::bug;
use rustc_middle::ty::{
    self, GenericArgsRef, Term, TermKind, Ty, TyCtxt, TypeSuperVisitable, TypeVisitable,
    TypeVisitableExt, TypeVisitor,
};
