// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/ty/impls_ty.rs
// Error: expected square brackets
// Problematic line: line 9


use rustc_data_structures::fingerprint::Fingerprint;
use rustc_data_structures::fx::FxHashMap;
use rustc_data_structures::stable_hasher::{
    HashStable, HashingControls, StableHasher, ToStableHashKey,
};
use rustc_query_system::ich::StableHashingContext;
