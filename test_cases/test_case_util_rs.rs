// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/ty/util.rs
// Error: expected square brackets
// Problematic line: line 28

use crate::mir;
use crate::query::Providers;
use crate::ty::layout::{FloatExt, IntegerExt};
use crate::ty::{
    self, Asyncness, FallibleTypeFolder, GenericArgKind, GenericArgsRef, Ty, TyCtxt, TypeFoldable,
    TypeFolder, TypeSuperFoldable, TypeVisitableExt, Upcast,
};
