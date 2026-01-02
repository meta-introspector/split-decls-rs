// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_borrowck/src/nll.rs
// Error: expected square brackets
// Problematic line: line 25

use crate::consumers::RustcFacts;
use crate::diagnostics::RegionErrors;
use crate::handle_placeholders::compute_sccs_applying_placeholder_outlives_constraints;
use crate::polonius::legacy::{
    PoloniusFacts, PoloniusFactsExt, PoloniusLocationTable, PoloniusOutput,
};
use crate::polonius::{PoloniusContext, PoloniusDiagnosticsContext};
