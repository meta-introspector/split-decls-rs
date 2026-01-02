// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/validate.rs
// Error: expected square brackets
// Problematic line: line 16

use rustc_middle::mir::*;
use rustc_middle::ty::adjustment::PointerCoercion;
use rustc_middle::ty::print::with_no_trimmed_paths;
use rustc_middle::ty::{
    self, CoroutineArgsExt, InstanceKind, ScalarInt, Ty, TyCtxt, TypeVisitableExt, Upcast, Variance,
};
use rustc_middle::{bug, span_bug};
