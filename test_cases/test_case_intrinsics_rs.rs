// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_const_eval/src/interpret/intrinsics.rs
// Error: expected square brackets
// Problematic line: line 19


use super::memory::MemoryKind;
use super::util::ensure_monomorphic_enough;
use super::{
    AllocId, CheckInAllocMsg, ImmTy, InterpCx, InterpResult, Machine, OpTy, PlaceTy, Pointer,
    PointerArithmetic, Provenance, Scalar, err_ub_custom, err_unsup_format, interp_ok, throw_inval,
    throw_ub_custom, throw_ub_format,
