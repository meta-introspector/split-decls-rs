// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_data_structures/src/base_n.rs
// Error: expected square brackets
// Problematic line: line 6


use std::{ascii, fmt};

#[cfg(test)]
mod tests;

pub const MAX_BASE: usize = 64;
