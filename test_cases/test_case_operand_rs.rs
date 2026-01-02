// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_const_eval/src/interpret/operand.rs
// Error: expected square brackets
// Problematic line: line 19

use tracing::field::Empty;
use tracing::trace;

use super::{
    CtfeProvenance, Frame, InterpCx, InterpResult, MPlaceTy, Machine, MemPlace, MemPlaceMeta,
    OffsetMode, PlaceTy, Pointer, Projectable, Provenance, Scalar, alloc_range, err_ub,
    from_known_layout, interp_ok, mir_assign_valid_types, throw_ub,
