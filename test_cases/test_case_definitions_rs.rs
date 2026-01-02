// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir/src/definitions.rs
// Error: expected square brackets
// Problematic line: line 22

use crate::def_id::{CRATE_DEF_INDEX, CrateNum, DefIndex, LOCAL_CRATE, LocalDefId, StableCrateId};
use crate::def_path_hash_map::DefPathHashMap;

/// The `DefPathTable` maps `DefIndex`es to `DefKey`s and vice versa.
/// Internally the `DefPathTable` holds a tree of `DefKey`s, where each `DefKey`
/// stores the `DefIndex` of its parent.
/// There is one `DefPathTable` for each crate.
