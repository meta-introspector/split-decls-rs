// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_query_system/src/dep_graph/dep_node.rs
// Error: expected square brackets
// Problematic line: line 70

use super::{DepContext, FingerprintStyle, SerializedDepNodeIndex};
use crate::ich::StableHashingContext;

/// This serves as an index into arrays built by `make_dep_kind_array`.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct DepKind {
    variant: u16,
