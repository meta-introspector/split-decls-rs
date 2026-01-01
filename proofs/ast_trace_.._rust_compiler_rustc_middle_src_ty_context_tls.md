# AST Trace: ../rust/compiler/rustc_middle/src/ty/context/tls.rs

Generated 12 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use std::{mem, ptr};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
use rustc_data_structures::sync;

use super::{GlobalCtxt, TyCtxt};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=STRUCT | NAME=ImplicitCtxt | COMPLEXITY=5 | LINES=24

```rust
use crate::dep_graph::TaskDepsRef;
use crate::query::plumbing::QueryJobId;

/// This is the implicit state of rustc. It contains the current
/// `TyCtxt` and query. It is updated when creating a local interner or
/// executing a new query. Whenever there's a `TyCtxt` value available
/// you should also have access to an `ImplicitCtxt` through the functions
/// in this module.
#[derive(Clone)]
pub struct ImplicitCtxt<'a, 'tcx> {
    /// The current `TyCtxt`.
    pub tcx: TyCtxt<'tcx>,

    /// The current query job, if any. This is updated by `JobOwner::start` in
    /// `ty::query::plumbing` when executing a query.
    pub query: Option<QueryJobId>,

    /// Used to prevent queries from calling too deeply.
    pub query_depth: usize,

    /// The current dep graph task. This is used to add dependencies to queries
    /// when executing them.
    pub task_deps: TaskDepsRef<'a>,
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=new | COMPLEXITY=5 | LINES=7

```rust
impl<'a, 'tcx> ImplicitCtxt<'a, 'tcx> {
    pub fn new(gcx: &'tcx GlobalCtxt<'tcx>) -> Self {
        let tcx = TyCtxt { gcx };
        ImplicitCtxt { tcx, query: None, query_depth: 0, task_deps: TaskDepsRef::Ignore }
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=erase | COMPLEXITY=4 | LINES=8

```rust
// Import the thread-local variable from Rayon, which is preserved for Rayon jobs.
use rustc_thread_pool::tlv::TLV;

#[inline]
fn erase(context: &ImplicitCtxt<'_, '_>) -> *const () {
    context as *const _ as *const ()
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=11 | LINES=5

```rust
#[inline]
unsafe fn downcast<'a, 'tcx>(context: *const ()) -> &'a ImplicitCtxt<'a, 'tcx> {
    unsafe { &*(context as *const ImplicitCtxt<'a, 'tcx>) }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=enter_context | COMPLEXITY=5 | LINES=13

```rust
/// Sets `context` as the new current `ImplicitCtxt` for the duration of the function `f`.
#[inline]
pub fn enter_context<'a, 'tcx, F, R>(context: &ImplicitCtxt<'a, 'tcx>, f: F) -> R
where
    F: FnOnce() -> R,
{
    TLV.with(|tlv| {
        let old = tlv.replace(erase(context));
        let _reset = rustc_data_structures::defer(move || tlv.set(old));
        f()
    })
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=with_context_opt | COMPLEXITY=14 | LINES=19

```rust
/// Allows access to the current `ImplicitCtxt` in a closure if one is available.
#[inline]
#[track_caller]
pub fn with_context_opt<F, R>(f: F) -> R
where
    F: for<'a, 'tcx> FnOnce(Option<&ImplicitCtxt<'a, 'tcx>>) -> R,
{
    let context = TLV.get();
    if context.is_null() {
        f(None)
    } else {
        // We could get an `ImplicitCtxt` pointer from another thread.
        // Ensure that `ImplicitCtxt` is `DynSync`.
        sync::assert_dyn_sync::<ImplicitCtxt<'_, '_>>();

        unsafe { f(Some(downcast(context))) }
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=with_context | COMPLEXITY=4 | LINES=10

```rust
/// Allows access to the current `ImplicitCtxt`.
/// Panics if there is no `ImplicitCtxt` available.
#[inline]
pub fn with_context<F, R>(f: F) -> R
where
    F: for<'a, 'tcx> FnOnce(&ImplicitCtxt<'a, 'tcx>) -> R,
{
    with_context_opt(|opt_context| f(opt_context.expect("no ImplicitCtxt stored in tls")))
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=with_related_context | COMPLEXITY=13 | LINES=23

```rust
/// Allows access to the current `ImplicitCtxt` whose tcx field is the same as the tcx argument
/// passed in. This means the closure is given an `ImplicitCtxt` with the same `'tcx` lifetime
/// as the `TyCtxt` passed in.
/// This will panic if you pass it a `TyCtxt` which is different from the current
/// `ImplicitCtxt`'s `tcx` field.
#[inline]
pub fn with_related_context<'tcx, F, R>(tcx: TyCtxt<'tcx>, f: F) -> R
where
    F: FnOnce(&ImplicitCtxt<'_, 'tcx>) -> R,
{
    with_context(|context| {
        // The two gcx have different invariant lifetimes, so we need to erase them for the comparison.
        assert!(ptr::eq(
            context.tcx.gcx as *const _ as *const (),
            tcx.gcx as *const _ as *const ()
        ));

        let context: &ImplicitCtxt<'_, '_> = unsafe { mem::transmute(context) };

        f(context)
    })
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=with | COMPLEXITY=4 | LINES=10

```rust
/// Allows access to the `TyCtxt` in the current `ImplicitCtxt`.
/// Panics if there is no `ImplicitCtxt` available.
#[inline]
pub fn with<F, R>(f: F) -> R
where
    F: for<'tcx> FnOnce(TyCtxt<'tcx>) -> R,
{
    with_context(|context| f(context.tcx))
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=with_opt | COMPLEXITY=4 | LINES=14

```rust
/// Allows access to the `TyCtxt` in the current `ImplicitCtxt`.
/// The closure is passed None if there is no `ImplicitCtxt` available.
#[inline]
#[track_caller]
pub fn with_opt<F, R>(f: F) -> R
where
    F: for<'tcx> FnOnce(Option<TyCtxt<'tcx>>) -> R,
{
    with_context_opt(
        #[track_caller]
        |opt_context| f(opt_context.map(|context| context.tcx)),
    )
}
```

---
*Generated by AST tracing system*
