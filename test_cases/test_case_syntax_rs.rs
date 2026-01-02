// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/mir/syntax.rs
// Error: expected square brackets
// Problematic line: line 24

use crate::ty::adjustment::PointerCoercion;
use crate::ty::{self, GenericArgsRef, List, Region, Ty, UserTypeAnnotationIndex};

/// Represents the "flavors" of MIR.
///
/// The MIR pipeline is structured into a few major dialects, with one or more phases within each
/// dialect. A MIR flavor is identified by a dialect-phase pair. A single `MirPhase` value
