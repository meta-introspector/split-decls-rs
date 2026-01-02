// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_serialize/src/serialize.rs
// Error: expected square brackets
// Problematic line: line 17

use smallvec::{Array, SmallVec};
use thin_vec::ThinVec;

/// A byte that [cannot occur in UTF8 sequences][utf8]. Used to mark the end of a string.
/// This way we can skip validation and still be relatively sure that deserialization
/// did not desynchronize.
///
