// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/mir/interpret/allocation.rs
// Error: expected square brackets
// Problematic line: line 21

use rustc_macros::HashStable;
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};

use super::{
    AllocId, BadBytesAccess, CtfeProvenance, InterpErrorKind, InterpResult, Pointer,
    PointerArithmetic, Provenance, ResourceExhaustionInfo, Scalar, ScalarSizeMismatch,
    UndefinedBehaviorInfo, UnsupportedOpInfo, interp_ok, read_target_uint, write_target_uint,
