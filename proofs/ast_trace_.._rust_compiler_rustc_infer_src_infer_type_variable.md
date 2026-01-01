# AST Trace: ../rust/compiler/rustc_infer/src/infer/type_variable.rs

Generated 23 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
use std::cmp;
use std::marker::PhantomData;
use std::ops::Range;

use rustc_data_structures::undo_log::Rollback;
use rustc_data_structures::{snapshot_vec as sv, unify as ut};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
use rustc_hir::def_id::DefId;
use rustc_index::IndexVec;
use rustc_middle::bug;
use rustc_middle::ty::{self, Ty, TyVid};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
use rustc_span::Span;
use tracing::debug;

use crate::infer::InferCtxtUndoLogs;

/// Represents a single undo-able action that affects a type inference variable.
#[derive(Clone)]
pub(crate) enum UndoLog<'tcx> {
    EqRelation(sv::UndoLog<ut::Delegate<TyVidEqKey<'tcx>>>),
    SubRelation(sv::UndoLog<ut::Delegate<TyVidSubKey>>),
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=7

```rust
/// Convert from a specific kind of undo to the more general UndoLog
impl<'tcx> From<sv::UndoLog<ut::Delegate<TyVidEqKey<'tcx>>>> for UndoLog<'tcx> {
    fn from(l: sv::UndoLog<ut::Delegate<TyVidEqKey<'tcx>>>) -> Self {
        UndoLog::EqRelation(l)
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=7

```rust
/// Convert from a specific kind of undo to the more general UndoLog
impl<'tcx> From<sv::UndoLog<ut::Delegate<TyVidSubKey>>> for UndoLog<'tcx> {
    fn from(l: sv::UndoLog<ut::Delegate<TyVidSubKey>>) -> Self {
        UndoLog::SubRelation(l)
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=reverse | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx> Rollback<sv::UndoLog<ut::Delegate<TyVidEqKey<'tcx>>>> for TypeVariableStorage<'tcx> {
    fn reverse(&mut self, undo: sv::UndoLog<ut::Delegate<TyVidEqKey<'tcx>>>) {
        self.eq_relations.reverse(undo)
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=reverse | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx> Rollback<sv::UndoLog<ut::Delegate<TyVidSubKey>>> for TypeVariableStorage<'tcx> {
    fn reverse(&mut self, undo: sv::UndoLog<ut::Delegate<TyVidSubKey>>) {
        self.sub_unification_table.reverse(undo)
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=reverse | COMPLEXITY=9 | LINES=9

```rust
impl<'tcx> Rollback<UndoLog<'tcx>> for TypeVariableStorage<'tcx> {
    fn reverse(&mut self, undo: UndoLog<'tcx>) {
        match undo {
            UndoLog::EqRelation(undo) => self.eq_relations.reverse(undo),
            UndoLog::SubRelation(undo) => self.sub_unification_table.reverse(undo),
        }
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=14 | LINES=29

```rust
#[derive(Clone, Default)]
pub(crate) struct TypeVariableStorage<'tcx> {
    /// The origins of each type variable.
    values: IndexVec<TyVid, TypeVariableData>,
    /// Two variables are unified in `eq_relations` when we have a
    /// constraint `?X == ?Y`. This table also stores, for each key,
    /// the known value.
    eq_relations: ut::UnificationTableStorage<TyVidEqKey<'tcx>>,
    /// Only used by `-Znext-solver` and for diagnostics. Tracks whether
    /// type variables are related via subtyping at all, ignoring which of
    /// the two is the subtype.
    ///
    /// When reporting ambiguity errors, we sometimes want to
    /// treat all inference vars which are subtypes of each
    /// others as if they are equal. For this case we compute
    /// the transitive closure of our subtype obligations here.
    ///
    /// E.g. when encountering ambiguity errors, we want to suggest
    /// specifying some method argument or to add a type annotation
    /// to a local variable. Because subtyping cannot change the
    /// shape of a type, it's fine if the cause of the ambiguity error
    /// is only related to the suggested variable via subtyping.
    ///
    /// Even for something like `let x = returns_arg(); x.method();` the
    /// type of `x` is only a supertype of the argument of `returns_arg`. We
    /// still want to suggest specifying the type of the argument.
    sub_unification_table: ut::UnificationTableStorage<TyVidSubKey>,
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
pub(crate) struct TypeVariableTable<'a, 'tcx> {
    storage: &'a mut TypeVariableStorage<'tcx>,

    undo_log: &'a mut InferCtxtUndoLogs<'tcx>,
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=STRUCT | NAME=TypeVariableOrigin | COMPLEXITY=6 | LINES=9

```rust
#[derive(Copy, Clone, Debug)]
pub struct TypeVariableOrigin {
    pub span: Span,
    /// `DefId` of the type parameter this was instantiated for, if any.
    ///
    /// This should only be used for diagnostics.
    pub param_def_id: Option<DefId>,
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
#[derive(Clone)]
pub(crate) struct TypeVariableData {
    origin: TypeVariableOrigin,
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=6

```rust
#[derive(Copy, Clone, Debug)]
pub(crate) enum TypeVariableValue<'tcx> {
    Known { value: Ty<'tcx> },
    Unknown { universe: ty::UniverseIndex },
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=17 | LINES=18

```rust
impl<'tcx> TypeVariableValue<'tcx> {
    /// If this value is known, returns the type it is known to be.
    /// Otherwise, `None`.
    pub(crate) fn known(&self) -> Option<Ty<'tcx>> {
        match *self {
            TypeVariableValue::Unknown { .. } => None,
            TypeVariableValue::Known { value } => Some(value),
        }
    }

    pub(crate) fn is_unknown(&self) -> bool {
        match *self {
            TypeVariableValue::Unknown { .. } => true,
            TypeVariableValue::Known { .. } => false,
        }
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=7 | LINES=20

```rust
impl<'tcx> TypeVariableStorage<'tcx> {
    #[inline]
    pub(crate) fn with_log<'a>(
        &'a mut self,
        undo_log: &'a mut InferCtxtUndoLogs<'tcx>,
    ) -> TypeVariableTable<'a, 'tcx> {
        TypeVariableTable { storage: self, undo_log }
    }

    #[inline]
    pub(crate) fn eq_relations_ref(&self) -> &ut::UnificationTableStorage<TyVidEqKey<'tcx>> {
        &self.eq_relations
    }

    pub(super) fn finalize_rollback(&mut self) {
        debug_assert!(self.values.len() >= self.eq_relations.len());
        self.values.truncate(self.eq_relations.len());
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=eq_relations | COMPLEXITY=53 | LINES=144

```rust
impl<'tcx> TypeVariableTable<'_, 'tcx> {
    /// Returns the origin that was given when `vid` was created.
    ///
    /// Note that this function does not return care whether
    /// `vid` has been unified with something else or not.
    pub(crate) fn var_origin(&self, vid: ty::TyVid) -> TypeVariableOrigin {
        self.storage.values[vid].origin
    }

    /// Records that `a == b`.
    ///
    /// Precondition: neither `a` nor `b` are known.
    pub(crate) fn equate(&mut self, a: ty::TyVid, b: ty::TyVid) {
        debug_assert!(self.probe(a).is_unknown());
        debug_assert!(self.probe(b).is_unknown());
        self.eq_relations().union(a, b);
        self.sub_unification_table().union(a, b);
    }

    /// Records that `a` and `b` are related via subtyping. We don't track
    /// which of the two is the subtype.
    ///
    /// Precondition: neither `a` nor `b` are known.
    pub(crate) fn sub_unify(&mut self, a: ty::TyVid, b: ty::TyVid) {
        debug_assert!(self.probe(a).is_unknown());
        debug_assert!(self.probe(b).is_unknown());
        self.sub_unification_table().union(a, b);
    }

    /// Instantiates `vid` with the type `ty`.
    ///
    /// Precondition: `vid` must not have been previously instantiated.
    pub(crate) fn instantiate(&mut self, vid: ty::TyVid, ty: Ty<'tcx>) {
        let vid = self.root_var(vid);
        debug_assert!(!ty.is_ty_var(), "instantiating ty var with var: {vid:?} {ty:?}");
        debug_assert!(self.probe(vid).is_unknown());
        debug_assert!(
            self.eq_relations().probe_value(vid).is_unknown(),
            "instantiating type variable `{vid:?}` twice: new-value = {ty:?}, old-value={:?}",
            self.eq_relations().probe_value(vid)
        );
        self.eq_relations().union_value(vid, TypeVariableValue::Known { value: ty });
    }

    /// Creates a new type variable.
    ///
    /// - `diverging`: indicates if this is a "diverging" type
    ///   variable, e.g.,  one created as the type of a `return`
    ///   expression. The code in this module doesn't care if a
    ///   variable is diverging, but the main Rust type-checker will
    ///   sometimes "unify" such variables with the `!` or `()` types.
    /// - `origin`: indicates *why* the type variable was created.
    ///   The code in this module doesn't care, but it can be useful
    ///   for improving error messages.
    pub(crate) fn new_var(
        &mut self,
        universe: ty::UniverseIndex,
        origin: TypeVariableOrigin,
    ) -> ty::TyVid {
        let eq_key = self.eq_relations().new_key(TypeVariableValue::Unknown { universe });

        let sub_key = self.sub_unification_table().new_key(());
        debug_assert_eq!(eq_key.vid, sub_key.vid);

        let index = self.storage.values.push(TypeVariableData { origin });
        debug_assert_eq!(eq_key.vid, index);

        debug!("new_var(index={:?}, universe={:?}, origin={:?})", eq_key.vid, universe, origin);

        index
    }

    /// Returns the number of type variables created thus far.
    pub(crate) fn num_vars(&self) -> usize {
        self.storage.values.len()
    }

    /// Returns the "root" variable of `vid` in the `eq_relations`
    /// equivalence table. All type variables that have been equated
    /// will yield the same root variable (per the union-find
    /// algorithm), so `root_var(a) == root_var(b)` implies that `a ==
    /// b` (transitively).
    pub(crate) fn root_var(&mut self, vid: ty::TyVid) -> ty::TyVid {
        self.eq_relations().find(vid).vid
    }

    /// Returns the "root" variable of `vid` in the `sub_unification_table`
    /// equivalence table. All type variables that have been are related via
    /// equality or subtyping will yield the same root variable (per the
    /// union-find algorithm), so `sub_unification_table_root_var(a)
    /// == sub_unification_table_root_var(b)` implies that:
    /// ```text
    /// exists X. (a <: X || X <: a) && (b <: X || X <: b)
    /// ```
    pub(crate) fn sub_unification_table_root_var(&mut self, vid: ty::TyVid) -> ty::TyVid {
        self.sub_unification_table().find(vid).vid
    }

    /// Retrieves the type to which `vid` has been instantiated, if
    /// any.
    pub(crate) fn probe(&mut self, vid: ty::TyVid) -> TypeVariableValue<'tcx> {
        self.inlined_probe(vid)
    }

    /// An always-inlined variant of `probe`, for very hot call sites.
    #[inline(always)]
    pub(crate) fn inlined_probe(&mut self, vid: ty::TyVid) -> TypeVariableValue<'tcx> {
        self.eq_relations().inlined_probe_value(vid)
    }

    #[inline]
    fn eq_relations(&mut self) -> super::UnificationTable<'_, 'tcx, TyVidEqKey<'tcx>> {
        self.storage.eq_relations.with_log(self.undo_log)
    }

    #[inline]
    fn sub_unification_table(&mut self) -> super::UnificationTable<'_, 'tcx, TyVidSubKey> {
        self.storage.sub_unification_table.with_log(self.undo_log)
    }

    /// Returns a range of the type variables created during the snapshot.
    pub(crate) fn vars_since_snapshot(
        &mut self,
        value_count: usize,
    ) -> (Range<TyVid>, Vec<TypeVariableOrigin>) {
        let range = TyVid::from_usize(value_count)..TyVid::from_usize(self.num_vars());
        (range.clone(), range.map(|index| self.var_origin(index)).collect())
    }

    /// Returns indices of all variables that are not yet
    /// instantiated.
    pub(crate) fn unresolved_variables(&mut self) -> Vec<ty::TyVid> {
        (0..self.num_vars())
            .filter_map(|i| {
                let vid = ty::TyVid::from_usize(i);
                match self.probe(vid) {
                    TypeVariableValue::Unknown { .. } => Some(vid),
                    TypeVariableValue::Known { .. } => None,
                }
            })
            .collect()
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=13

```rust
///////////////////////////////////////////////////////////////////////////

/// These structs (a newtyped TyVid) are used as the unification key
/// for the `eq_relations`; they carry a `TypeVariableValue` along
/// with them.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct TyVidEqKey<'tcx> {
    vid: ty::TyVid,

    // in the table, we map each ty-vid to one of these:
    phantom: PhantomData<TypeVariableValue<'tcx>>,
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=from | COMPLEXITY=8 | LINES=7

```rust
impl<'tcx> From<ty::TyVid> for TyVidEqKey<'tcx> {
    #[inline] // make this function eligible for inlining - it is quite hot.
    fn from(vid: ty::TyVid) -> Self {
        TyVidEqKey { vid, phantom: PhantomData }
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=index | COMPLEXITY=13 | LINES=18

```rust
impl<'tcx> ut::UnifyKey for TyVidEqKey<'tcx> {
    type Value = TypeVariableValue<'tcx>;
    #[inline(always)]
    fn index(&self) -> u32 {
        self.vid.as_u32()
    }
    #[inline]
    fn from_index(i: u32) -> Self {
        TyVidEqKey::from(ty::TyVid::from_u32(i))
    }
    fn tag() -> &'static str {
        "TyVidEqKey"
    }
    fn order_roots(a: Self, _: &Self::Value, b: Self, _: &Self::Value) -> Option<(Self, Self)> {
        if a.vid.as_u32() < b.vid.as_u32() { Some((a, b)) } else { Some((b, a)) }
    }
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct TyVidSubKey {
    vid: ty::TyVid,
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=from | COMPLEXITY=8 | LINES=7

```rust
impl From<ty::TyVid> for TyVidSubKey {
    #[inline] // make this function eligible for inlining - it is quite hot.
    fn from(vid: ty::TyVid) -> Self {
        TyVidSubKey { vid }
    }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=index | COMPLEXITY=8 | LINES=15

```rust
impl ut::UnifyKey for TyVidSubKey {
    type Value = ();
    #[inline]
    fn index(&self) -> u32 {
        self.vid.as_u32()
    }
    #[inline]
    fn from_index(i: u32) -> TyVidSubKey {
        TyVidSubKey { vid: ty::TyVid::from_u32(i) }
    }
    fn tag() -> &'static str {
        "TyVidSubKey"
    }
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=unify_values | COMPLEXITY=23 | LINES=33

```rust
impl<'tcx> ut::UnifyValue for TypeVariableValue<'tcx> {
    type Error = ut::NoError;

    fn unify_values(value1: &Self, value2: &Self) -> Result<Self, ut::NoError> {
        match (value1, value2) {
            // We never equate two type variables, both of which
            // have known types. Instead, we recursively equate
            // those types.
            (&TypeVariableValue::Known { .. }, &TypeVariableValue::Known { .. }) => {
                bug!("equating two type variables, both of which have known types")
            }

            // If one side is known, prefer that one.
            (&TypeVariableValue::Known { .. }, &TypeVariableValue::Unknown { .. }) => Ok(*value1),
            (&TypeVariableValue::Unknown { .. }, &TypeVariableValue::Known { .. }) => Ok(*value2),

            // If both sides are *unknown*, it hardly matters, does it?
            (
                &TypeVariableValue::Unknown { universe: universe1 },
                &TypeVariableValue::Unknown { universe: universe2 },
            ) => {
                // If we unify two unbound variables, ?T and ?U, then whatever
                // value they wind up taking (which must be the same value) must
                // be nameable by both universes. Therefore, the resulting
                // universe is the minimum of the two universes, because that is
                // the one which contains the fewest names in scope.
                let universe = cmp::min(universe1, universe2);
                Ok(TypeVariableValue::Unknown { universe })
            }
        }
    }
}
```

---
*Generated by AST tracing system*
