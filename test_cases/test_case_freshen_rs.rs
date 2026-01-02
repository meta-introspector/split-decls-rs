// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_infer/src/infer/freshen.rs
// Error: expected square brackets
// Problematic line: line 38


use rustc_data_structures::fx::FxHashMap;
use rustc_middle::bug;
use rustc_middle::ty::{
    self, Ty, TyCtxt, TypeFoldable, TypeFolder, TypeSuperFoldable, TypeVisitableExt,
};

