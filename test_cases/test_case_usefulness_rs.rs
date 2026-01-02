// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_pattern_analysis/src/usefulness.rs
// Error: expected square brackets
// Problematic line: line 713


use std::fmt;

#[cfg(feature = "rustc")]
use rustc_data_structures::stack::ensure_sufficient_stack;
use rustc_hash::{FxHashMap, FxHashSet};
use rustc_index::bit_set::DenseBitSet;
