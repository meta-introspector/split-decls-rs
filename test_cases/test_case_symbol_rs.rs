// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_span/src/symbol.rs
// Error: expected square brackets
// Problematic line: line 11


use rustc_arena::DroplessArena;
use rustc_data_structures::fx::{FxHashSet, FxIndexSet};
use rustc_data_structures::stable_hasher::{
    HashStable, StableCompare, StableHasher, ToStableHashKey,
};
use rustc_data_structures::sync::Lock;
