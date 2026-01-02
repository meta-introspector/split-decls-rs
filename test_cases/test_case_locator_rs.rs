// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_metadata/src/locator.rs
// Error: expected square brackets
// Problematic line: line 241

use crate::errors;
use crate::rmeta::{METADATA_HEADER, MetadataBlob, rustc_version};

#[derive(Clone)]
pub(crate) struct CrateLocator<'a> {
    // Immutable per-session configuration.
    only_needs_metadata: bool,
