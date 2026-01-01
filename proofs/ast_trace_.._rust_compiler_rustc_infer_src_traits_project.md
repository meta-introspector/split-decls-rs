# AST Trace: ../rust/compiler/rustc_infer/src/traits/project.rs

Generated 13 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
// Code for projecting associated types out of trait references.

use crate::rustc_data_structures::snapshot_map::{self, SnapshotMapRef, SnapshotMapStorage};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
use crate::rustc_data_structures::undo_log::Rollback;
use crate::rustc_complete::traits::EvaluationResult;
use crate::rustc_complete::ty;
use tracing::{debug, info};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=STRUCT | NAME=MismatchedProjectionTypes | COMPLEXITY=2 | LINES=11

```rust
use super::PredicateObligations;
use crate::infer::snapshot::undo_log::InferCtxtUndoLogs;

pub(crate) type UndoLog<'tcx> =
    snapshot_map::UndoLog<ProjectionCacheKey<'tcx>, ProjectionCacheEntry<'tcx>>;

#[derive(Clone)]
pub struct MismatchedProjectionTypes<'tcx> {
    pub err: ty::error::TypeError<'tcx>,
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=STRUCT | NAME=Normalized | COMPLEXITY=2 | LINES=6

```rust
#[derive(Clone)]
pub struct Normalized<'tcx, T> {
    pub value: T,
    pub obligations: PredicateObligations<'tcx>,
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=with | COMPLEXITY=4 | LINES=8

```rust
pub type NormalizedTerm<'tcx> = Normalized<'tcx, ty::Term<'tcx>>;

impl<'tcx, T> Normalized<'tcx, T> {
    pub fn with<U>(self, value: U) -> Normalized<'tcx, U> {
        Normalized { value, obligations: self.obligations }
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=STRUCT | NAME=ProjectionCache | COMPLEXITY=11 | LINES=36

```rust
// # Cache

/// The projection cache. Unlike the standard caches, this can include
/// infcx-dependent type variables, therefore we have to roll the
/// cache back each time we roll a snapshot back, to avoid assumptions
/// on yet-unresolved inference variables. Types with placeholder
/// regions also have to be removed when the respective snapshot ends.
///
/// Because of that, projection cache entries can be "stranded" and left
/// inaccessible when type variables inside the key are resolved. We make no
/// attempt to recover or remove "stranded" entries, but rather let them be
/// (for the lifetime of the infcx).
///
/// Entries in the projection cache might contain inference variables
/// that will be resolved by obligations on the projection cache entry (e.g.,
/// when a type parameter in the associated type is constrained through
/// an "RFC 447" projection on the impl).
///
/// When working with a fulfillment context, the derived obligations of each
/// projection cache entry will be registered on the fulfillcx, so any users
/// that can wait for a fulfillcx fixed point need not care about this. However,
/// users that don't wait for a fixed point (e.g., trait evaluation) have to
/// resolve the obligations themselves to make sure the projected result is
/// ok and avoid issues like #43132.
///
/// If that is done, after evaluation the obligations, it is a good idea to
/// call `ProjectionCache::complete` to make sure the obligations won't be
/// re-evaluated and avoid an exponential worst-case.
//
// FIXME: we probably also want some sort of cross-infcx cache here to
// reduce the amount of duplication. Let's see what we get with the Chalk reforms.
pub struct ProjectionCache<'a, 'tcx> {
    map: &'a mut SnapshotMapStorage<ProjectionCacheKey<'tcx>, ProjectionCacheEntry<'tcx>>,
    undo_log: &'a mut InferCtxtUndoLogs<'tcx>,
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=STRUCT | NAME=ProjectionCacheStorage | COMPLEXITY=2 | LINES=5

```rust
#[derive(Clone, Default)]
pub struct ProjectionCacheStorage<'tcx> {
    map: SnapshotMapStorage<ProjectionCacheKey<'tcx>, ProjectionCacheEntry<'tcx>>,
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=STRUCT | NAME=ProjectionCacheKey | COMPLEXITY=2 | LINES=6

```rust
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct ProjectionCacheKey<'tcx> {
    term: ty::AliasTerm<'tcx>,
    param_env: ty::ParamEnv<'tcx>,
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=new | COMPLEXITY=4 | LINES=6

```rust
impl<'tcx> ProjectionCacheKey<'tcx> {
    pub fn new(term: ty::AliasTerm<'tcx>, param_env: ty::ParamEnv<'tcx>) -> Self {
        Self { term, param_env }
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=10 | LINES=37

```rust
#[derive(Clone, Debug)]
pub enum ProjectionCacheEntry<'tcx> {
    InProgress,
    Ambiguous,
    Recur,
    Error,
    NormalizedTerm {
        ty: NormalizedTerm<'tcx>,
        /// If we were able to successfully evaluate the corresponding cache
        /// entry key during predicate evaluation, then this field stores the
        /// final result obtained from evaluating all of the projection
        /// sub-obligations. During evaluation, we will skip evaluating the
        /// cached sub-obligations in `ty` if this field is set. Evaluation
        /// only cares about the final result, so we don't care about any
        /// region constraint side-effects produced by evaluating the
        /// sub-obligations.
        ///
        /// Additionally, we will clear out the sub-obligations entirely if we
        /// ever evaluate the cache entry (along with all its sub obligations)
        /// to `EvaluatedToOk`. This affects all users of the cache, not just
        /// evaluation. Since a result of `EvaluatedToOk` means that there were
        /// no region obligations that need to be tracked, it's fine to forget
        /// about the sub-obligations - they don't provide any additional
        /// information. However, we do *not* discard any obligations when we
        /// see `EvaluatedToOkModuloRegions` - we don't know which
        /// sub-obligations may introduce region constraints, so we keep them
        /// all to be safe.
        ///
        /// When we are not performing evaluation (e.g. in
        /// `FulfillmentContext`), we ignore this field, and always re-process
        /// the cached sub-obligations (which may have been cleared out - see
        /// the above paragraph). This ensures that we do not lose any regions
        /// constraints that arise from processing the sub-obligations.
        complete: Option<EvaluationResult>,
    },
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=10

```rust
impl<'tcx> ProjectionCacheStorage<'tcx> {
    #[inline]
    pub(crate) fn with_log<'a>(
        &'a mut self,
        undo_log: &'a mut InferCtxtUndoLogs<'tcx>,
    ) -> ProjectionCache<'a, 'tcx> {
        ProjectionCache { map: &mut self.map, undo_log }
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=map | COMPLEXITY=58 | LINES=107

```rust
impl<'tcx> ProjectionCache<'_, 'tcx> {
    #[inline]
    fn map(
        &mut self,
    ) -> SnapshotMapRef<
        '_,
        ProjectionCacheKey<'tcx>,
        ProjectionCacheEntry<'tcx>,
        InferCtxtUndoLogs<'tcx>,
    > {
        self.map.with_log(self.undo_log)
    }

    pub fn clear(&mut self) {
        self.map().clear();
    }

    /// Try to start normalize `key`; returns an error if
    /// normalization already occurred (this error corresponds to a
    /// cache hit, so it's actually a good thing).
    pub fn try_start(
        &mut self,
        key: ProjectionCacheKey<'tcx>,
    ) -> Result<(), ProjectionCacheEntry<'tcx>> {
        let mut map = self.map();
        if let Some(entry) = map.get(&key) {
            return Err(entry.clone());
        }

        map.insert(key, ProjectionCacheEntry::InProgress);
        Ok(())
    }

    /// Indicates that `key` was normalized to `value`.
    pub fn insert_term(&mut self, key: ProjectionCacheKey<'tcx>, value: NormalizedTerm<'tcx>) {
        debug!(
            "ProjectionCacheEntry::insert_ty: adding cache entry: key={:?}, value={:?}",
            key, value
        );
        let mut map = self.map();
        if let Some(ProjectionCacheEntry::Recur) = map.get(&key) {
            debug!("Not overwriting Recur");
            return;
        }
        let fresh_key =
            map.insert(key, ProjectionCacheEntry::NormalizedTerm { ty: value, complete: None });
        assert!(!fresh_key, "never started projecting `{key:?}`");
    }

    /// Mark the relevant projection cache key as having its derived obligations
    /// complete, so they won't have to be re-computed (this is OK to do in a
    /// snapshot - if the snapshot is rolled back, the obligations will be
    /// marked as incomplete again).
    pub fn complete(&mut self, key: ProjectionCacheKey<'tcx>, result: EvaluationResult) {
        let mut map = self.map();
        match map.get(&key) {
            Some(ProjectionCacheEntry::NormalizedTerm { ty, complete: _ }) => {
                info!("ProjectionCacheEntry::complete({:?}) - completing {:?}", key, ty);
                let mut ty = ty.clone();
                if result.must_apply_considering_regions() {
                    ty.obligations = PredicateObligations::new();
                }
                map.insert(
                    key,
                    ProjectionCacheEntry::NormalizedTerm { ty, complete: Some(result) },
                );
            }
            ref value => {
                // Type inference could "strand behind" old cache entries. Leave
                // them alone for now.
                info!("ProjectionCacheEntry::complete({:?}) - ignoring {:?}", key, value);
            }
        };
    }

    pub fn is_complete(&mut self, key: ProjectionCacheKey<'tcx>) -> Option<EvaluationResult> {
        self.map().get(&key).and_then(|res| match res {
            ProjectionCacheEntry::NormalizedTerm { ty: _, complete } => *complete,
            _ => None,
        })
    }

    /// Indicates that trying to normalize `key` resulted in
    /// ambiguity. No point in trying it again then until we gain more
    /// type information (in which case, the "fully resolved" key will
    /// be different).
    pub fn ambiguous(&mut self, key: ProjectionCacheKey<'tcx>) {
        let fresh = self.map().insert(key, ProjectionCacheEntry::Ambiguous);
        assert!(!fresh, "never started projecting `{key:?}`");
    }

    /// Indicates that while trying to normalize `key`, `key` was required to
    /// be normalized again. Selection or evaluation should eventually report
    /// an error here.
    pub fn recur(&mut self, key: ProjectionCacheKey<'tcx>) {
        let fresh = self.map().insert(key, ProjectionCacheEntry::Recur);
        assert!(!fresh, "never started projecting `{key:?}`");
    }

    /// Indicates that trying to normalize `key` resulted in
    /// error.
    pub fn error(&mut self, key: ProjectionCacheKey<'tcx>) {
        let fresh = self.map().insert(key, ProjectionCacheEntry::Error);
        assert!(!fresh, "never started projecting `{key:?}`");
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=reverse | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx> Rollback<UndoLog<'tcx>> for ProjectionCacheStorage<'tcx> {
    fn reverse(&mut self, undo: UndoLog<'tcx>) {
        self.map.reverse(undo);
    }
}
```

---
*Generated by AST tracing system*
