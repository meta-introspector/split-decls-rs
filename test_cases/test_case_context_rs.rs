// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/ty/context.rs
// Error: expected square brackets
// Problematic line: line 29

use rustc_data_structures::sharded::{IntoPointer, ShardedHashMap};
use rustc_data_structures::stable_hasher::{HashStable, StableHasher};
use rustc_data_structures::steal::Steal;
use rustc_data_structures::sync::{
    self, DynSend, DynSync, FreezeReadGuard, Lock, RwLock, WorkerLocal,
};
use rustc_errors::{
