// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_dataflow/src/framework/cursor.rs
// Error: expected square brackets
// Problematic line: line 7

use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};

#[cfg(debug_assertions)]
use rustc_index::bit_set::DenseBitSet;
use rustc_middle::mir::{self, BasicBlock, Location};

