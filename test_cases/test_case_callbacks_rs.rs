// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_interface/src/callbacks.rs
// Error: expected square brackets
// Problematic line: line 21

use rustc_query_system::dep_graph::dep_node::default_dep_kind_debug;
use rustc_query_system::dep_graph::{DepContext, DepKind, DepNode};

fn track_span_parent(def_id: rustc_span::def_id::LocalDefId) {
    tls::with_context_opt(|icx| {
        if let Some(icx) = icx {
            // `track_span_parent` gets called a lot from HIR lowering code.
