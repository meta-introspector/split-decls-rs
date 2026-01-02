// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_const_eval/src/interpret/call.rs
// Error: expected square brackets
// Problematic line: line 17

use tracing::field::Empty;
use tracing::{info, instrument, trace};

use super::{
    CtfeProvenance, FnVal, ImmTy, InterpCx, InterpResult, MPlaceTy, Machine, OpTy, PlaceTy,
    Projectable, Provenance, ReturnAction, ReturnContinuation, Scalar, StackPopInfo, interp_ok,
    throw_ub, throw_ub_custom, throw_unsup_format,
