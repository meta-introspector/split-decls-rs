// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_incremental/src/persist/dirty_clean.rs
// Error: expected square brackets
// Problematic line: line 26

use rustc_data_structures::fx::FxHashSet;
use rustc_data_structures::unord::UnordSet;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::{
    Attribute, ImplItemKind, ItemKind as HirItem, Node as HirNode, TraitItemKind, intravisit,
};
use rustc_middle::dep_graph::{DepNode, DepNodeExt, label_strs};
