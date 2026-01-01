# AST Trace: ../rust/compiler/rustc_infer/src/infer/outlives/test_type_match.rs

Generated 9 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
use std::collections::hash_map::Entry;

use rustc_data_structures::fx::FxHashMap;
use rustc_middle::ty::error::TypeError;
use rustc_middle::ty::{self, Ty, TyCtxt, TypeVisitableExt};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
use tracing::instrument;

use crate::infer::region_constraints::VerifyIfEq;
use crate::infer::relate::{self as relate, Relate, RelateResult, TypeRelation};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
/// Given a "verify-if-eq" type test like:
///
/// ```rust,ignore (pseudo-Rust)
/// exists<'a...> {
///     verify_if_eq(some_type, bound_region)
/// }
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=extract_verify_if_eq | COMPLEXITY=21 | LINES=53

```rust
/// ```
///
/// and the type `test_ty` that the type test is being tested against,
/// returns:
///
/// * `None` if `some_type` cannot be made equal to `test_ty`,
///   no matter the values of the variables in `exists`.
/// * `Some(r)` with a suitable bound (typically the value of `bound_region`, modulo
///   any bound existential variables, which will be instantiated) for the
///   type under test.
///
/// NB: This function uses a simplistic, syntactic version of type equality.
/// In other words, it may spuriously return `None` even if the type-under-test
/// is in fact equal to `some_type`. In practice, though, this is used on types
/// that are either projections like `T::Item` or `T` and it works fine, but it
/// could have trouble when complex types with higher-ranked binders and the
/// like are used. This is a particular challenge since this function is invoked
/// very late in inference and hence cannot make use of the normal inference
/// machinery.
#[instrument(level = "debug", skip(tcx))]
pub fn extract_verify_if_eq<'tcx>(
    tcx: TyCtxt<'tcx>,
    verify_if_eq_b: &ty::Binder<'tcx, VerifyIfEq<'tcx>>,
    test_ty: Ty<'tcx>,
) -> Option<ty::Region<'tcx>> {
    assert!(!verify_if_eq_b.has_escaping_bound_vars());
    let mut m = MatchAgainstHigherRankedOutlives::new(tcx);
    let verify_if_eq = verify_if_eq_b.skip_binder();
    m.relate(verify_if_eq.ty, test_ty).ok()?;

    if let ty::RegionKind::ReBound(depth, br) = verify_if_eq.bound.kind() {
        assert!(depth == ty::INNERMOST);
        match m.map.get(&br) {
            Some(&r) => Some(r),
            None => {
                // If there is no mapping, then this region is unconstrained.
                // In that case, we escalate to `'static`.
                Some(tcx.lifetimes.re_static)
            }
        }
    } else {
        // The region does not contain any bound variables, so we don't need
        // to do any instantiation.
        //
        // Example:
        //
        // for<'a> <T as Foo<'a>>::Item: 'b
        //
        // In this case, we've now matched and found a value for
        // `'a`, but it doesn't affect the bound `'b`.
        Some(verify_if_eq.bound)
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=9 | LINES=18

```rust
/// True if a (potentially higher-ranked) outlives
#[instrument(level = "debug", skip(tcx))]
pub(super) fn can_match_erased_ty<'tcx>(
    tcx: TyCtxt<'tcx>,
    outlives_predicate: ty::Binder<'tcx, ty::TypeOutlivesPredicate<'tcx>>,
    erased_ty: Ty<'tcx>,
) -> bool {
    assert!(!outlives_predicate.has_escaping_bound_vars());
    let erased_outlives_predicate = tcx.erase_and_anonymize_regions(outlives_predicate);
    let outlives_ty = erased_outlives_predicate.skip_binder().0;
    if outlives_ty == erased_ty {
        // pointless micro-optimization
        true
    } else {
        MatchAgainstHigherRankedOutlives::new(tcx).relate(outlives_ty, erased_ty).is_ok()
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=STRUCT | NAME=MatchAgainstHigherRankedOutlives | COMPLEXITY=2 | LINES=6

```rust
struct MatchAgainstHigherRankedOutlives<'tcx> {
    tcx: TyCtxt<'tcx>,
    pattern_depth: ty::DebruijnIndex,
    map: FxHashMap<ty::BoundRegion, ty::Region<'tcx>>,
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=new | COMPLEXITY=4 | LINES=10

```rust
impl<'tcx> MatchAgainstHigherRankedOutlives<'tcx> {
    fn new(tcx: TyCtxt<'tcx>) -> MatchAgainstHigherRankedOutlives<'tcx> {
        MatchAgainstHigherRankedOutlives {
            tcx,
            pattern_depth: ty::INNERMOST,
            map: FxHashMap::default(),
        }
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=no_match | COMPLEXITY=17 | LINES=30

```rust
impl<'tcx> MatchAgainstHigherRankedOutlives<'tcx> {
    /// Creates the "Error" variant that signals "no match".
    fn no_match<T>(&self) -> RelateResult<'tcx, T> {
        Err(TypeError::Mismatch)
    }

    /// Binds the pattern variable `br` to `value`; returns an `Err` if the pattern
    /// is already bound to a different value.
    #[instrument(level = "trace", skip(self))]
    fn bind(
        &mut self,
        br: ty::BoundRegion,
        value: ty::Region<'tcx>,
    ) -> RelateResult<'tcx, ty::Region<'tcx>> {
        match self.map.entry(br) {
            Entry::Occupied(entry) => {
                if *entry.get() == value {
                    Ok(value)
                } else {
                    self.no_match()
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(value);
                Ok(value)
            }
        }
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=cx | COMPLEXITY=36 | LINES=77

```rust
impl<'tcx> TypeRelation<TyCtxt<'tcx>> for MatchAgainstHigherRankedOutlives<'tcx> {
    fn cx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }

    #[instrument(level = "trace", skip(self))]
    fn relate_with_variance<T: Relate<TyCtxt<'tcx>>>(
        &mut self,
        variance: ty::Variance,
        _: ty::VarianceDiagInfo<TyCtxt<'tcx>>,
        a: T,
        b: T,
    ) -> RelateResult<'tcx, T> {
        // Opaque types args have lifetime parameters.
        // We must not check them to be equal, as we never insert anything to make them so.
        if variance != ty::Bivariant { self.relate(a, b) } else { Ok(a) }
    }

    #[instrument(skip(self), level = "trace")]
    fn regions(
        &mut self,
        pattern: ty::Region<'tcx>,
        value: ty::Region<'tcx>,
    ) -> RelateResult<'tcx, ty::Region<'tcx>> {
        if let ty::RegionKind::ReBound(depth, br) = pattern.kind()
            && depth == self.pattern_depth
        {
            self.bind(br, value)
        } else if pattern == value {
            Ok(pattern)
        } else {
            self.no_match()
        }
    }

    #[instrument(skip(self), level = "trace")]
    fn tys(&mut self, pattern: Ty<'tcx>, value: Ty<'tcx>) -> RelateResult<'tcx, Ty<'tcx>> {
        // FIXME(non_lifetime_binders): What to do here?
        if matches!(pattern.kind(), ty::Error(_) | ty::Bound(..)) {
            // Unlike normal `TypeRelation` rules, `ty::Error` does not equal any type.
            self.no_match()
        } else if pattern == value {
            Ok(pattern)
        } else {
            relate::structurally_relate_tys(self, pattern, value)
        }
    }

    #[instrument(skip(self), level = "trace")]
    fn consts(
        &mut self,
        pattern: ty::Const<'tcx>,
        value: ty::Const<'tcx>,
    ) -> RelateResult<'tcx, ty::Const<'tcx>> {
        if pattern == value {
            Ok(pattern)
        } else {
            relate::structurally_relate_consts(self, pattern, value)
        }
    }

    #[instrument(skip(self), level = "trace")]
    fn binders<T>(
        &mut self,
        pattern: ty::Binder<'tcx, T>,
        value: ty::Binder<'tcx, T>,
    ) -> RelateResult<'tcx, ty::Binder<'tcx, T>>
    where
        T: Relate<TyCtxt<'tcx>>,
    {
        self.pattern_depth.shift_in(1);
        let result = Ok(pattern.rebind(self.relate(pattern.skip_binder(), value.skip_binder())?));
        self.pattern_depth.shift_out(1);
        result
    }
}
```

---
*Generated by AST tracing system*
