// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_build/src/builder/expr/as_place.rs
// Error: expected square brackets
// Problematic line: line 21

use crate::builder::expr::category::Category;
use crate::builder::{BlockAnd, BlockAndExtension, Builder, Capture, CaptureMap};

/// The "outermost" place that holds this value.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum PlaceBase {
    /// Denotes the start of a `Place`.
