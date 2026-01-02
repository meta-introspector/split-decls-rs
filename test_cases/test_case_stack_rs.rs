// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_const_eval/src/interpret/stack.rs
// Error: expected square brackets
// Problematic line: line 18

use tracing::field::Empty;
use tracing::{info_span, instrument, trace};

use super::{
    AllocId, CtfeProvenance, Immediate, InterpCx, InterpResult, Machine, MemPlace, MemPlaceMeta,
    MemoryKind, Operand, PlaceTy, Pointer, Provenance, ReturnAction, Scalar, from_known_layout,
    interp_ok, throw_ub, throw_unsup,
