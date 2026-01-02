// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_type_ir/src/solve/inspect.rs
// Error: expected square brackets
// Problematic line: line 26

use crate::solve::{CandidateSource, Certainty, Goal, GoalSource, QueryResult};
use crate::{Canonical, CanonicalVarValues, Interner};

/// Some `data` together with information about how they relate to the input
/// of the canonical query.
///
/// This is only ever used as [CanonicalState]. Any type information in proof
