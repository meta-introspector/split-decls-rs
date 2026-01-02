// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_next_trait_solver/src/solve/assembly/structural_traits.rs
// Error: expected square brackets
// Problematic line: line 10

use rustc_type_ir::lang_items::{SolverLangItem, SolverTraitLangItem};
use rustc_type_ir::solve::SizedTraitKind;
use rustc_type_ir::solve::inspect::ProbeKind;
use rustc_type_ir::{
    self as ty, FallibleTypeFolder, Interner, Movability, Mutability, TypeFoldable,
    TypeSuperFoldable, Upcast as _, elaborate,
};
