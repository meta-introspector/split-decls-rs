# AST Trace: ../rust/compiler/rustc_interface/src/callbacks.rs

Generated 9 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=7 | LINES=14

```rust
// Throughout the compiler tree, there are several places which want to have
// access to state or queries while being inside crates that are dependencies
// of `rustc_middle`. To facilitate this, we have the
// `crate::rustc_data_structures::AtomicRef` type, which allows us to setup a global
// static which can then be set in this file at program startup.
//
// See `SPAN_TRACK` for an example of how to set things up.
//
// The functions in this file should fall back to the default set in their
// origin crate when the `TyCtxt` is not present in TLS.

use std::fmt;

use crate::rustc_complete::{DiagInner, TRACK_DIAGNOSTIC};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::rustc_complete::dep_graph::{DepNodeExt, TaskDepsRef};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
use crate::rustc_complete::ty::tls;
use rustc_query_impl::QueryCtxt;
use rustc_query_system::dep_graph::dep_node::default_dep_kind_debug;
use rustc_query_system::dep_graph::{DepContext, DepKind, DepNode};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=track_span_parent | COMPLEXITY=16 | LINES=18

```rust
fn track_span_parent(def_id: crate::rustc_span::def_id::LocalDefId) {
    tls::with_context_opt(|icx| {
        if let Some(icx) = icx {
            // `track_span_parent` gets called a lot from HIR lowering code.
            // Skip doing anything if we aren't tracking dependencies.
            let tracks_deps = match icx.task_deps {
                TaskDepsRef::Allow(..) => true,
                TaskDepsRef::EvalAlways | TaskDepsRef::Ignore | TaskDepsRef::Forbid => false,
            };
            if tracks_deps {
                let _span = icx.tcx.source_span(def_id);
                // Sanity check: relative span's parent must be an absolute span.
                debug_assert_eq!(_span.data_untracked().parent, None);
            }
        }
    })
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=track_diagnostic | COMPLEXITY=11 | LINES=18

```rust
/// This is a callback from `rustc_errors` as it cannot access the implicit state
/// in `rustc_middle` otherwise. It is used when diagnostic messages are
/// emitted and stores them in the current query, if there is one.
fn track_diagnostic<R>(diagnostic: DiagInner, f: &mut dyn FnMut(DiagInner) -> R) -> R {
    tls::with_context_opt(|icx| {
        if let Some(icx) = icx {
            icx.tcx.dep_graph.record_diagnostic(QueryCtxt::new(icx.tcx), &diagnostic);

            // Diagnostics are tracked, we can ignore the dependency.
            let icx = tls::ImplicitCtxt { task_deps: TaskDepsRef::Ignore, ..icx.clone() };
            tls::enter_context(&icx, move || (*f)(diagnostic))
        } else {
            // In any other case, invoke diagnostics anyway.
            (*f)(diagnostic)
        }
    })
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=def_id_debug | COMPLEXITY=9 | LINES=13

```rust
/// This is a callback from `rustc_hir` as it cannot access the implicit state
/// in `rustc_middle` otherwise.
fn def_id_debug(def_id: crate::rustc_hir::def_id::DefId, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "DefId({}:{}", def_id.krate, def_id.index.index())?;
    tls::with_opt(|opt_tcx| {
        if let Some(tcx) = opt_tcx {
            write!(f, " ~ {}", tcx.def_path_debug_str(def_id))?;
        }
        Ok(())
    })?;
    write!(f, ")")
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=dep_kind_debug | COMPLEXITY=8 | LINES=12

```rust
/// This is a callback from `rustc_query_system` as it cannot access the implicit state
/// in `rustc_middle` otherwise.
pub fn dep_kind_debug(kind: DepKind, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    tls::with_opt(|opt_tcx| {
        if let Some(tcx) = opt_tcx {
            write!(f, "{}", tcx.dep_kind_info(kind).name)
        } else {
            default_dep_kind_debug(kind, f)
        }
    })
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=dep_node_debug | COMPLEXITY=20 | LINES=23

```rust
/// This is a callback from `rustc_query_system` as it cannot access the implicit state
/// in `rustc_middle` otherwise.
pub fn dep_node_debug(node: DepNode, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}(", node.kind)?;

    tls::with_opt(|opt_tcx| {
        if let Some(tcx) = opt_tcx {
            if let Some(def_id) = node.extract_def_id(tcx) {
                write!(f, "{}", tcx.def_path_debug_str(def_id))?;
            } else if let Some(ref s) = tcx.dep_graph.dep_node_debug_str(node) {
                write!(f, "{s}")?;
            } else {
                write!(f, "{}", node.hash)?;
            }
        } else {
            write!(f, "{}", node.hash)?;
        }
        Ok(())
    })?;

    write!(f, ")")
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=setup_callbacks | COMPLEXITY=3 | LINES=12

```rust
/// Sets up the callbacks in prior crates which we want to refer to the
/// TyCtxt in.
pub fn setup_callbacks() {
    crate::rustc_span::SPAN_TRACK.swap(&(track_span_parent as fn(_)));
    crate::rustc_hir::def_id::DEF_ID_DEBUG.swap(&(def_id_debug as fn(_, &mut fmt::Formatter<'_>) -> _));
    rustc_query_system::dep_graph::dep_node::DEP_KIND_DEBUG
        .swap(&(dep_kind_debug as fn(_, &mut fmt::Formatter<'_>) -> _));
    rustc_query_system::dep_graph::dep_node::DEP_NODE_DEBUG
        .swap(&(dep_node_debug as fn(_, &mut fmt::Formatter<'_>) -> _));
    TRACK_DIAGNOSTIC.swap(&(track_diagnostic as _));
}
```

---
*Generated by AST tracing system*
