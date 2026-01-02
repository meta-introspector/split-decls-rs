// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_feature/src/unstable.rs
// Error: expected square brackets
// Problematic line: line 11


use super::{Feature, to_nonzero};

#[derive(PartialEq)]
enum FeatureStatus {
    Default,
    Incomplete,
