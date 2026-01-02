// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_const_eval/src/interpret/machine.rs
// Error: expected square brackets
// Problematic line: line 18

use rustc_span::def_id::DefId;
use rustc_target::callconv::FnAbi;

use super::{
    AllocBytes, AllocId, AllocKind, AllocRange, Allocation, CTFE_ALLOC_SALT, ConstAllocation,
    CtfeProvenance, EnteredTraceSpan, FnArg, Frame, ImmTy, InterpCx, InterpResult, MPlaceTy,
    MemoryKind, Misalignment, OpTy, PlaceTy, Pointer, Provenance, RangeSet, interp_ok, throw_unsup,
