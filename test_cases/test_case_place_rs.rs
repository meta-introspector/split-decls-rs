// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_const_eval/src/interpret/place.rs
// Error: expected square brackets
// Problematic line: line 15

use tracing::field::Empty;
use tracing::{instrument, trace};

use super::{
    AllocInit, AllocRef, AllocRefMut, CheckAlignMsg, CtfeProvenance, ImmTy, Immediate, InterpCx,
    InterpResult, Machine, MemoryKind, Misalignment, OffsetMode, OpTy, Operand, Pointer,
    Projectable, Provenance, Scalar, alloc_range, interp_ok, mir_assign_valid_types,
