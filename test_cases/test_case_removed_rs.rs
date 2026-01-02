// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_feature/src/removed.rs
// Error: expected square brackets
// Problematic line: line 9


use super::{Feature, to_nonzero};

pub struct RemovedFeature {
    pub feature: Feature,
    pub reason: Option<&'static str>,
    pub pull: Option<NonZero<u32>>,
