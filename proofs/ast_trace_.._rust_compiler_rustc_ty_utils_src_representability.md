# AST Trace: ../rust/compiler/rustc_ty_utils/src/representability.rs

Generated 8 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
use crate::rustc_complete::def::DefKind;
use crate::rustc_index::bit_set::DenseBitSet;
use crate::rustc_complete::bug;
use crate::rustc_complete::query::Providers;
use crate::rustc_complete::ty::{self, Representability, Ty, TyCtxt};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=3 | LINES=6

```rust
use crate::rustc_complete::def_id::LocalDefId;

pub(crate) fn provide(providers: &mut Providers) {
    *providers =
        Providers { representability, representability_adt_ty, params_in_repr, ..*providers };
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=13 | LINES=9

```rust
macro_rules! rtry {
    ($e:expr) => {
        match $e {
            e @ Representability::Infinite(_) => return e,
            Representability::Representable => {}
        }
    };
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=representability | COMPLEXITY=15 | LINES=15

```rust
fn representability(tcx: TyCtxt<'_>, def_id: LocalDefId) -> Representability {
    match tcx.def_kind(def_id) {
        DefKind::Struct | DefKind::Union | DefKind::Enum => {
            for variant in tcx.adt_def(def_id).variants() {
                for field in variant.fields.iter() {
                    rtry!(tcx.representability(field.did.expect_local()));
                }
            }
            Representability::Representable
        }
        DefKind::Field => representability_ty(tcx, tcx.type_of(def_id).instantiate_identity()),
        def_kind => bug!("unexpected {def_kind:?}"),
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=representability_ty | COMPLEXITY=11 | LINES=15

```rust
fn representability_ty<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Representability {
    match *ty.kind() {
        ty::Adt(..) => tcx.representability_adt_ty(ty),
        // FIXME(#11924) allow zero-length arrays?
        ty::Array(ty, _) => representability_ty(tcx, ty),
        ty::Tuple(tys) => {
            for ty in tys {
                rtry!(representability_ty(tcx, ty));
            }
            Representability::Representable
        }
        _ => Representability::Representable,
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=representability_adt_ty | COMPLEXITY=22 | LINES=34

```rust
/*
The reason for this being a separate query is very subtle:
Consider this infinitely sized struct: `struct Foo(Box<Foo>, Bar<Foo>)`:
When calling representability(Foo), a query cycle will occur:
  representability(Foo)
    -> representability_adt_ty(Bar<Foo>)
    -> representability(Foo)
For the diagnostic output (in `Value::from_cycle_error`), we want to detect that
the `Foo` in the *second* field of the struct is culpable. This requires
traversing the HIR of the struct and calling `params_in_repr(Bar)`. But we can't
call params_in_repr for a given type unless it is known to be representable.
params_in_repr will cycle/panic on infinitely sized types. Looking at the query
cycle above, we know that `Bar` is representable because
representability_adt_ty(Bar<..>) is in the cycle and representability(Bar) is
*not* in the cycle.
*/
fn representability_adt_ty<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Representability {
    let ty::Adt(adt, args) = ty.kind() else { bug!("expected adt") };
    if let Some(def_id) = adt.did().as_local() {
        rtry!(tcx.representability(def_id));
    }
    // At this point, we know that the item of the ADT type is representable;
    // but the type parameters may cause a cycle with an upstream type
    let params_in_repr = tcx.params_in_repr(adt.did());
    for (i, arg) in args.iter().enumerate() {
        if let ty::GenericArgKind::Type(ty) = arg.kind() {
            if params_in_repr.contains(i as u32) {
                rtry!(representability_ty(tcx, ty));
            }
        }
    }
    Representability::Representable
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=params_in_repr | COMPLEXITY=9 | LINES=16

```rust
fn params_in_repr(tcx: TyCtxt<'_>, def_id: LocalDefId) -> DenseBitSet<u32> {
    let adt_def = tcx.adt_def(def_id);
    let generics = tcx.generics_of(def_id);
    let mut params_in_repr = DenseBitSet::new_empty(generics.own_params.len());
    for variant in adt_def.variants() {
        for field in variant.fields.iter() {
            params_in_repr_ty(
                tcx,
                tcx.type_of(field.did).instantiate_identity(),
                &mut params_in_repr,
            );
        }
    }
    params_in_repr
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=params_in_repr_ty | COMPLEXITY=19 | LINES=21

```rust
fn params_in_repr_ty<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>, params_in_repr: &mut DenseBitSet<u32>) {
    match *ty.kind() {
        ty::Adt(adt, args) => {
            let inner_params_in_repr = tcx.params_in_repr(adt.did());
            for (i, arg) in args.iter().enumerate() {
                if let ty::GenericArgKind::Type(ty) = arg.kind() {
                    if inner_params_in_repr.contains(i as u32) {
                        params_in_repr_ty(tcx, ty, params_in_repr);
                    }
                }
            }
        }
        ty::Array(ty, _) => params_in_repr_ty(tcx, ty, params_in_repr),
        ty::Tuple(tys) => tys.iter().for_each(|ty| params_in_repr_ty(tcx, ty, params_in_repr)),
        ty::Param(param) => {
            params_in_repr.insert(param.index);
        }
        _ => {}
    }
}
```

---
*Generated by AST tracing system*
