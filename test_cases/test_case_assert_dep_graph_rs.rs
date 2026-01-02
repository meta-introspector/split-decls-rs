// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_incremental/src/assert_dep_graph.rs
// Error: expected square brackets
// Problematic line: line 44

use rustc_data_structures::graph::linked_graph::{Direction, INCOMING, NodeIndex, OUTGOING};
use rustc_hir::def_id::{CRATE_DEF_ID, DefId, LocalDefId};
use rustc_hir::intravisit::{self, Visitor};
use rustc_middle::dep_graph::{
    DepGraphQuery, DepKind, DepNode, DepNodeExt, DepNodeFilter, EdgeFilter, dep_kinds,
};
use rustc_middle::hir::nested_filter;
