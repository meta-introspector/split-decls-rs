// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_trait_selection/src/traits/select/confirmation.rs
// Error: expected square brackets
// Problematic line: line 27

use super::{PredicateObligations, SelectionContext};
use crate::traits::normalize::{normalize_with_depth, normalize_with_depth_to};
use crate::traits::util::{self, closure_trait_ref_and_return_type};
use crate::traits::{
    ImplSource, ImplSourceUserDefinedData, Normalized, Obligation, ObligationCause,
    PolyTraitObligation, PredicateObligation, Selection, SelectionError, TraitObligation,
};
