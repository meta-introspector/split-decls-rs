// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/middle/privacy.rs
// Error: expected square brackets
// Problematic line: line 16


use crate::ty::{TyCtxt, Visibility};

/// Represents the levels of effective visibility an item can have.
///
/// The variants are sorted in ascending order of directness.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, HashStable)]
