// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/mir/interpret/allocation/provenance_map.rs
// Error: expected square brackets
// Problematic line: line 16

use super::{AllocRange, CtfeProvenance, Provenance, alloc_range};
use crate::mir::interpret::{AllocError, AllocResult};

/// Stores the provenance information of pointers stored in memory.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
#[derive(HashStable)]
pub struct ProvenanceMap<Prov = CtfeProvenance> {
