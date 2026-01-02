// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_const_eval/src/interpret/memory.rs
// Error: expected square brackets
// Problematic line: line 23

use rustc_middle::{bug, throw_ub_format};
use tracing::{debug, instrument, trace};

use super::{
    AllocBytes, AllocId, AllocInit, AllocMap, AllocRange, Allocation, CheckAlignMsg,
    CheckInAllocMsg, CtfeProvenance, GlobalAlloc, InterpCx, InterpResult, Machine, MayLeak,
    Misalignment, Pointer, PointerArithmetic, Provenance, Scalar, alloc_range, err_ub,
