// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_data_structures/src/sync.rs
// Error: expected square brackets
// Problematic line: line 33

use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

pub use parking_lot::{
    MappedRwLockReadGuard as MappedReadGuard, MappedRwLockWriteGuard as MappedWriteGuard,
    RwLockReadGuard as ReadGuard, RwLockWriteGuard as WriteGuard,
};
