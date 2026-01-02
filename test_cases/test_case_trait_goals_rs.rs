// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_next_trait_solver/src/solve/trait_goals.rs
// Error: expected square brackets
// Problematic line: line 8

use rustc_type_ir::inherent::*;
use rustc_type_ir::lang_items::SolverTraitLangItem;
use rustc_type_ir::solve::{CanonicalResponse, SizedTraitKind};
use rustc_type_ir::{
    self as ty, Interner, Movability, PredicatePolarity, TraitPredicate, TraitRef,
    TypeVisitableExt as _, TypingMode, Upcast as _, elaborate,
};
