// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_infer/src/infer/canonical/canonicalizer.rs
// Error: expected square brackets
// Problematic line: line 12

use rustc_data_structures::sso::SsoHashMap;
use rustc_index::Idx;
use rustc_middle::bug;
use rustc_middle::ty::{
    self, BoundVar, GenericArg, InferConst, List, Ty, TyCtxt, TypeFlags, TypeFoldable, TypeFolder,
    TypeSuperFoldable, TypeVisitableExt,
};
