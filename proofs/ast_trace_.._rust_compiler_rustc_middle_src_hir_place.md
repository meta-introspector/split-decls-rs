# AST Trace: ../rust/compiler/rustc_middle/src/hir/place.rs

Generated 9 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use rustc_abi::{FieldIdx, VariantIdx};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use rustc_hir::HirId;
use rustc_macros::{HashStable, TyDecodable, TyEncodable, TypeFoldable, TypeVisitable};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=16

```rust
use crate::ty;
use crate::ty::Ty;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, TyEncodable, TyDecodable, HashStable)]
#[derive(TypeFoldable, TypeVisitable)]
pub enum PlaceBase {
    /// A temporary variable.
    Rvalue,
    /// A named `static` item.
    StaticItem,
    /// A named local variable.
    Local(HirId),
    /// An upvar referenced by closure env.
    Upvar(ty::UpvarId),
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=7 | LINES=30

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, TyEncodable, TyDecodable, HashStable)]
#[derive(TypeFoldable, TypeVisitable)]
pub enum ProjectionKind {
    /// A dereference of a pointer, reference or `Box<T>` of the given type.
    Deref,

    /// `B.F` where `B` is the base expression and `F` is
    /// the field. The field is identified by which variant
    /// it appears in along with a field index. The variant
    /// is used for enums.
    Field(FieldIdx, VariantIdx),

    /// Some index like `B[x]`, where `B` is the base
    /// expression. We don't preserve the index `x` because
    /// we won't need it.
    Index,

    /// A subslice covering a range of values like `B[x..y]`.
    Subslice,

    /// A conversion from an opaque type to its hidden type so we can
    /// do further projections on it.
    ///
    /// This is unused if `-Znext-solver` is enabled.
    OpaqueCast,

    /// `unwrap_binder!(expr)`
    UnwrapUnsafeBinder,
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=STRUCT | NAME=Projection | COMPLEXITY=2 | LINES=10

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, TyEncodable, TyDecodable, HashStable)]
#[derive(TypeFoldable, TypeVisitable)]
pub struct Projection<'tcx> {
    /// Type after the projection is applied.
    pub ty: Ty<'tcx>,

    /// Defines the kind of access made by the projection.
    pub kind: ProjectionKind,
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=STRUCT | NAME=Place | COMPLEXITY=3 | LINES=17

```rust
/// A `Place` represents how a value is located in memory. This does not
/// always correspond to a syntactic place expression. For example, when
/// processing a pattern, a `Place` can be used to refer to the sub-value
/// currently being inspected.
///
/// This is an HIR version of [`rustc_middle::mir::Place`].
#[derive(Clone, Debug, PartialEq, Eq, Hash, TyEncodable, TyDecodable, HashStable)]
#[derive(TypeFoldable, TypeVisitable)]
pub struct Place<'tcx> {
    /// The type of the `PlaceBase`
    pub base_ty: Ty<'tcx>,
    /// The "outermost" place that holds this value.
    pub base: PlaceBase,
    /// How this place is derived from the base place.
    pub projections: Vec<Projection<'tcx>>,
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=STRUCT | NAME=PlaceWithHirId | COMPLEXITY=3 | LINES=15

```rust
/// A `PlaceWithHirId` represents how a value is located in memory. This does not
/// always correspond to a syntactic place expression. For example, when
/// processing a pattern, a `Place` can be used to refer to the sub-value
/// currently being inspected.
///
/// This is an HIR version of [`rustc_middle::mir::Place`].
#[derive(Clone, Debug, PartialEq, Eq, Hash, TyEncodable, TyDecodable, HashStable)]
pub struct PlaceWithHirId<'tcx> {
    /// `HirId` of the expression or pattern producing this value.
    pub hir_id: HirId,

    /// Information about the `Place`.
    pub place: Place<'tcx>,
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=new | COMPLEXITY=5 | LINES=11

```rust
impl<'tcx> PlaceWithHirId<'tcx> {
    pub fn new(
        hir_id: HirId,
        base_ty: Ty<'tcx>,
        base: PlaceBase,
        projections: Vec<Projection<'tcx>>,
    ) -> PlaceWithHirId<'tcx> {
        PlaceWithHirId { hir_id, place: Place { base_ty, base, projections } }
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=deref_tys | COMPLEXITY=16 | LINES=30

```rust
impl<'tcx> Place<'tcx> {
    /// Returns an iterator of the types that have to be dereferenced to access
    /// the `Place`.
    ///
    /// The types are in the reverse order that they are applied. So if
    /// `x: &*const u32` and the `Place` is `**x`, then the types returned are
    ///`*const u32` then `&*const u32`.
    pub fn deref_tys(&self) -> impl Iterator<Item = Ty<'tcx>> {
        self.projections.iter().enumerate().rev().filter_map(move |(index, proj)| {
            if ProjectionKind::Deref == proj.kind {
                Some(self.ty_before_projection(index))
            } else {
                None
            }
        })
    }

    /// Returns the type of this `Place` after all projections have been applied.
    pub fn ty(&self) -> Ty<'tcx> {
        self.projections.last().map_or(self.base_ty, |proj| proj.ty)
    }

    /// Returns the type of this `Place` immediately before `projection_index`th projection
    /// is applied.
    pub fn ty_before_projection(&self, projection_index: usize) -> Ty<'tcx> {
        assert!(projection_index < self.projections.len());
        if projection_index == 0 { self.base_ty } else { self.projections[projection_index - 1].ty }
    }
}
```

---
*Generated by AST tracing system*
