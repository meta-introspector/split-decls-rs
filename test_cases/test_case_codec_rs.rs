// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/ty/codec.rs
// Error: expected square brackets
// Problematic line: line 28

use crate::traits;
use crate::ty::{self, AdtDef, GenericArgsRef, Ty, TyCtxt};

/// The shorthand encoding uses an enum's variant index `usize`
/// and is offset by this value so it never matches a real variant.
/// This offset is also chosen so that the first byte is never < 0x80.
pub const SHORTHAND_OFFSET: usize = 0x80;
