# AST Trace: ../rust/compiler/rustc_middle/src/ty/inhabitedness/mod.rs

Generated 9 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=6 | LINES=15

```rust
//! This module contains logic for determining whether a type is inhabited or
//! uninhabited. The [`InhabitedPredicate`] type captures the minimum
//! information needed to determine whether a type is inhabited given a
//! `ParamEnv` and module ID.
//!
//! # Example
//! ```rust
//! #![feature(never_type)]
//! mod a {
//!     pub mod b {
//!         pub struct SecretlyUninhabited {
//!             _priv: !,
//!         }
//!     }
//! }
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=5 | LINES=9

```rust
//!
//! mod c {
//!     enum Void {}
//!     pub struct AlsoSecretlyUninhabited {
//!         _priv: Void,
//!     }
//!     mod d {
//!     }
//! }
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
//!
//! struct Foo {
//!     x: a::b::SecretlyUninhabited,
//!     y: c::AlsoSecretlyUninhabited,
//! }
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=5 | LINES=22

```rust
//! ```
//! In this code, the type `Foo` will only be visibly uninhabited inside the
//! modules `b`, `c` and `d`. Calling `inhabited_predicate` on `Foo` will
//! return `NotInModule(b) AND NotInModule(c)`.
//!
//! We need this information for pattern-matching on `Foo` or types that contain
//! `Foo`.
//!
//! # Example
//! ```ignore(illustrative)
//! let foo_result: Result<T, Foo> = ... ;
//! let Ok(t) = foo_result;
//! ```
//! This code should only compile in modules where the uninhabitedness of `Foo`
//! is visible.

use rustc_type_ir::TyKind::*;
use tracing::instrument;

use crate::query::Providers;
use crate::ty::context::TyCtxt;
use crate::ty::{self, DefId, Ty, TypeVisitableExt, VariantDef, Visibility};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=3 | LINES=8

```rust
pub mod inhabited_predicate;

pub use inhabited_predicate::InhabitedPredicate;

pub(crate) fn provide(providers: &mut Providers) {
    *providers = Providers { inhabited_predicate_adt, inhabited_predicate_type, ..*providers };
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=inhabited_predicate_adt | COMPLEXITY=9 | LINES=15

```rust
/// Returns an `InhabitedPredicate` that is generic over type parameters and
/// requires calling [`InhabitedPredicate::instantiate`]
fn inhabited_predicate_adt(tcx: TyCtxt<'_>, def_id: DefId) -> InhabitedPredicate<'_> {
    if let Some(def_id) = def_id.as_local() {
        if matches!(tcx.representability(def_id), ty::Representability::Infinite(_)) {
            return InhabitedPredicate::True;
        }
    }
    let adt = tcx.adt_def(def_id);
    InhabitedPredicate::any(
        tcx,
        adt.variants().iter().map(|variant| variant.inhabited_predicate(tcx, adt)),
    )
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=inhabited_predicate | COMPLEXITY=13 | LINES=26

```rust
impl<'tcx> VariantDef {
    /// Calculates the forest of `DefId`s from which this variant is visibly uninhabited.
    pub fn inhabited_predicate(
        &self,
        tcx: TyCtxt<'tcx>,
        adt: ty::AdtDef<'_>,
    ) -> InhabitedPredicate<'tcx> {
        debug_assert!(!adt.is_union());
        InhabitedPredicate::all(
            tcx,
            self.fields.iter().map(|field| {
                let pred = tcx.type_of(field.did).instantiate_identity().inhabited_predicate(tcx);
                if adt.is_enum() {
                    return pred;
                }
                match field.vis {
                    Visibility::Public => pred,
                    Visibility::Restricted(from) => {
                        pred.or(tcx, InhabitedPredicate::NotInModule(from))
                    }
                }
            }),
        )
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=inhabited_predicate | COMPLEXITY=43 | LINES=93

```rust
impl<'tcx> Ty<'tcx> {
    #[instrument(level = "debug", skip(tcx), ret)]
    pub fn inhabited_predicate(self, tcx: TyCtxt<'tcx>) -> InhabitedPredicate<'tcx> {
        debug_assert!(!self.has_infer());
        match self.kind() {
            // For now, unions are always considered inhabited
            Adt(adt, _) if adt.is_union() => InhabitedPredicate::True,
            // Non-exhaustive ADTs from other crates are always considered inhabited
            Adt(adt, _) if adt.variant_list_has_applicable_non_exhaustive() => {
                InhabitedPredicate::True
            }
            Never => InhabitedPredicate::False,
            Param(_) | Alias(ty::Inherent | ty::Projection | ty::Free, _) => {
                InhabitedPredicate::GenericType(self)
            }
            Alias(ty::Opaque, alias_ty) => {
                match alias_ty.def_id.as_local() {
                    // Foreign opaque is considered inhabited.
                    None => InhabitedPredicate::True,
                    // Local opaque type may possibly be revealed.
                    Some(local_def_id) => {
                        let key = ty::OpaqueTypeKey { def_id: local_def_id, args: alias_ty.args };
                        InhabitedPredicate::OpaqueType(key)
                    }
                }
            }
            Tuple(tys) if tys.is_empty() => InhabitedPredicate::True,
            // use a query for more complex cases
            Adt(..) | Array(..) | Tuple(_) => tcx.inhabited_predicate_type(self),
            // references and other types are inhabited
            _ => InhabitedPredicate::True,
        }
    }

    /// Checks whether a type is visibly uninhabited from a particular module.
    ///
    /// # Example
    /// ```
    /// #![feature(never_type)]
    /// # fn main() {}
    /// enum Void {}
    /// mod a {
    ///     pub mod b {
    ///         pub struct SecretlyUninhabited {
    ///             _priv: !,
    ///         }
    ///     }
    /// }
    ///
    /// mod c {
    ///     use super::Void;
    ///     pub struct AlsoSecretlyUninhabited {
    ///         _priv: Void,
    ///     }
    ///     mod d {
    ///     }
    /// }
    ///
    /// struct Foo {
    ///     x: a::b::SecretlyUninhabited,
    ///     y: c::AlsoSecretlyUninhabited,
    /// }
    /// ```
    /// In this code, the type `Foo` will only be visibly uninhabited inside the
    /// modules b, c and d. This effects pattern-matching on `Foo` or types that
    /// contain `Foo`.
    ///
    /// # Example
    /// ```ignore (illustrative)
    /// let foo_result: Result<T, Foo> = ... ;
    /// let Ok(t) = foo_result;
    /// ```
    /// This code should only compile in modules where the uninhabitedness of Foo is
    /// visible.
    pub fn is_inhabited_from(
        self,
        tcx: TyCtxt<'tcx>,
        module: DefId,
        typing_env: ty::TypingEnv<'tcx>,
    ) -> bool {
        self.inhabited_predicate(tcx).apply(tcx, typing_env, module)
    }

    /// Returns true if the type is uninhabited without regard to visibility
    pub fn is_privately_uninhabited(
        self,
        tcx: TyCtxt<'tcx>,
        typing_env: ty::TypingEnv<'tcx>,
    ) -> bool {
        !self.inhabited_predicate(tcx).apply_ignore_module(tcx, typing_env)
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=inhabited_predicate_type | COMPLEXITY=12 | LINES=21

```rust
/// N.B. this query should only be called through `Ty::inhabited_predicate`
fn inhabited_predicate_type<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> InhabitedPredicate<'tcx> {
    match *ty.kind() {
        Adt(adt, args) => tcx.inhabited_predicate_adt(adt.did()).instantiate(tcx, args),

        Tuple(tys) => {
            InhabitedPredicate::all(tcx, tys.iter().map(|ty| ty.inhabited_predicate(tcx)))
        }

        // If we can evaluate the array length before having a `ParamEnv`, then
        // we can simplify the predicate. This is an optimization.
        Array(ty, len) => match len.try_to_target_usize(tcx) {
            Some(0) => InhabitedPredicate::True,
            Some(1..) => ty.inhabited_predicate(tcx),
            None => ty.inhabited_predicate(tcx).or(tcx, InhabitedPredicate::ConstIsZero(len)),
        },

        _ => bug!("unexpected TyKind, use `Ty::inhabited_predicate`"),
    }
}
```

---
*Generated by AST tracing system*
