// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/query/keys.rs
// Error: expected square brackets
// Problematic line: line 18

use crate::ty::{self, GenericArg, GenericArgsRef, Ty, TyCtxt};
use crate::{mir, traits};

/// Placeholder for `CrateNum`'s "local" counterpart
#[derive(Copy, Clone, Debug)]
pub struct LocalCrate;

