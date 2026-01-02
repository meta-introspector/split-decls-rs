// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/ty/normalize_erasing_regions.rs
// Error: expected square brackets
// Problematic line: line 14

use tracing::{debug, instrument};

use crate::traits::query::NoSolution;
use crate::ty::{
    self, EarlyBinder, FallibleTypeFolder, GenericArgsRef, Ty, TyCtxt, TypeFoldable, TypeFolder,
    TypeVisitableExt,
};
