// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/mismatched_static_lifetime.rs
// Error: expected square brackets
// Problematic line: line 13

use tracing::debug;

use crate::error_reporting::infer::nice_region_error::NiceRegionError;
use crate::errors::{
    DoesNotOutliveStaticFromImpl, ImplicitStaticLifetimeSubdiag,
    IntroducesStaticBecauseUnmetLifetimeReq, MismatchedStaticLifetime, note_and_explain,
};
