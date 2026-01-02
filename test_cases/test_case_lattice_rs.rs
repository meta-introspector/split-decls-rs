// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_infer/src/infer/relate/lattice.rs
// Error: expected square brackets
// Problematic line: line 32

use crate::infer::{DefineOpaqueTypes, InferCtxt, SubregionOrigin, TypeTrace};
use crate::traits::{Obligation, PredicateObligations};

#[derive(Clone, Copy)]
pub(crate) enum LatticeOpKind {
    Glb,
    Lub,
