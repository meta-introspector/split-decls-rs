// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_const_eval/src/interpret/projection.rs
// Error: expected square brackets
// Problematic line: line 19

use rustc_middle::{bug, mir, span_bug, ty};
use tracing::{debug, instrument};

use super::{
    InterpCx, InterpResult, MPlaceTy, Machine, MemPlaceMeta, OpTy, Provenance, Scalar, err_ub,
    interp_ok, throw_ub, throw_unsup,
};
