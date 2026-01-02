// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_trait_selection/src/traits/project.rs
// Error: expected square brackets
// Problematic line: line 15

use rustc_middle::traits::select::OverflowError;
use rustc_middle::traits::{BuiltinImplSource, ImplSource, ImplSourceUserDefinedData};
use rustc_middle::ty::fast_reject::DeepRejectCtxt;
use rustc_middle::ty::{
    self, Term, Ty, TyCtxt, TypeFoldable, TypeVisitableExt, TypingMode, Upcast,
};
use rustc_middle::{bug, span_bug};
