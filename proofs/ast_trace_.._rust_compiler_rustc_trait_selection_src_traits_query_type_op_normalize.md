# AST Trace: ../rust/compiler/rustc_trait_selection/src/traits/query/type_op/normalize.rs

Generated 11 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
use std::fmt;

use rustc_middle::traits::ObligationCause;
use rustc_middle::traits::query::NoSolution;
pub use rustc_middle::traits::query::type_op::{DeeplyNormalize, Normalize};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use rustc_middle::ty::{self, Lift, ParamEnvAnd, Ty, TyCtxt, TypeFoldable, TypeVisitableExt};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use rustc_span::Span;

use crate::infer::canonical::{CanonicalQueryInput, CanonicalQueryResponse};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=try_fast_path | COMPLEXITY=12 | LINES=27

```rust
use crate::traits::ObligationCtxt;

impl<'tcx, T> super::QueryTypeOp<'tcx> for Normalize<T>
where
    T: Normalizable<'tcx> + 'tcx,
{
    type QueryResponse = T;

    fn try_fast_path(_tcx: TyCtxt<'tcx>, key: &ParamEnvAnd<'tcx, Self>) -> Option<T> {
        if !key.value.value.has_aliases() { Some(key.value.value) } else { None }
    }

    fn perform_query(
        tcx: TyCtxt<'tcx>,
        canonicalized: CanonicalQueryInput<'tcx, ParamEnvAnd<'tcx, Self>>,
    ) -> Result<CanonicalQueryResponse<'tcx, Self::QueryResponse>, NoSolution> {
        T::type_op_method(tcx, canonicalized)
    }

    fn perform_locally_with_next_solver(
        _ocx: &ObligationCtxt<'_, 'tcx>,
        key: ParamEnvAnd<'tcx, Self>,
        _span: Span,
    ) -> Result<Self::QueryResponse, NoSolution> {
        Ok(key.value.value)
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=try_fast_path | COMPLEXITY=17 | LINES=42

```rust
impl<'tcx, T> super::QueryTypeOp<'tcx> for DeeplyNormalize<T>
where
    T: Normalizable<'tcx> + 'tcx,
{
    type QueryResponse = T;

    fn try_fast_path(_tcx: TyCtxt<'tcx>, key: &ParamEnvAnd<'tcx, Self>) -> Option<T> {
        if !key.value.value.has_aliases() { Some(key.value.value) } else { None }
    }

    fn perform_query(
        tcx: TyCtxt<'tcx>,
        canonicalized: CanonicalQueryInput<'tcx, ParamEnvAnd<'tcx, Self>>,
    ) -> Result<CanonicalQueryResponse<'tcx, Self::QueryResponse>, NoSolution> {
        T::type_op_method(
            tcx,
            CanonicalQueryInput {
                typing_mode: canonicalized.typing_mode,
                canonical: canonicalized.canonical.unchecked_map(
                    |ty::ParamEnvAnd { param_env, value }| ty::ParamEnvAnd {
                        param_env,
                        value: Normalize { value: value.value },
                    },
                ),
            },
        )
    }

    fn perform_locally_with_next_solver(
        ocx: &ObligationCtxt<'_, 'tcx>,
        key: ParamEnvAnd<'tcx, Self>,
        span: Span,
    ) -> Result<Self::QueryResponse, NoSolution> {
        ocx.deeply_normalize(
            &ObligationCause::dummy_with_span(span),
            key.param_env,
            key.value.value,
        )
        .map_err(|_| NoSolution)
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=type_op_method | COMPLEXITY=2 | LINES=9

```rust
pub trait Normalizable<'tcx>:
    fmt::Debug + TypeFoldable<TyCtxt<'tcx>> + Lift<TyCtxt<'tcx>> + Copy
{
    fn type_op_method(
        tcx: TyCtxt<'tcx>,
        canonicalized: CanonicalQueryInput<'tcx, ParamEnvAnd<'tcx, Normalize<Self>>>,
    ) -> Result<CanonicalQueryResponse<'tcx, Self>, NoSolution>;
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=type_op_method | COMPLEXITY=5 | LINES=9

```rust
impl<'tcx> Normalizable<'tcx> for Ty<'tcx> {
    fn type_op_method(
        tcx: TyCtxt<'tcx>,
        canonicalized: CanonicalQueryInput<'tcx, ParamEnvAnd<'tcx, Normalize<Self>>>,
    ) -> Result<CanonicalQueryResponse<'tcx, Self>, NoSolution> {
        tcx.type_op_normalize_ty(canonicalized)
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=type_op_method | COMPLEXITY=5 | LINES=9

```rust
impl<'tcx> Normalizable<'tcx> for ty::Clause<'tcx> {
    fn type_op_method(
        tcx: TyCtxt<'tcx>,
        canonicalized: CanonicalQueryInput<'tcx, ParamEnvAnd<'tcx, Normalize<Self>>>,
    ) -> Result<CanonicalQueryResponse<'tcx, Self>, NoSolution> {
        tcx.type_op_normalize_clause(canonicalized)
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=type_op_method | COMPLEXITY=5 | LINES=9

```rust
impl<'tcx> Normalizable<'tcx> for ty::PolyFnSig<'tcx> {
    fn type_op_method(
        tcx: TyCtxt<'tcx>,
        canonicalized: CanonicalQueryInput<'tcx, ParamEnvAnd<'tcx, Normalize<Self>>>,
    ) -> Result<CanonicalQueryResponse<'tcx, Self>, NoSolution> {
        tcx.type_op_normalize_poly_fn_sig(canonicalized)
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=type_op_method | COMPLEXITY=5 | LINES=9

```rust
impl<'tcx> Normalizable<'tcx> for ty::FnSig<'tcx> {
    fn type_op_method(
        tcx: TyCtxt<'tcx>,
        canonicalized: CanonicalQueryInput<'tcx, ParamEnvAnd<'tcx, Normalize<Self>>>,
    ) -> Result<CanonicalQueryResponse<'tcx, Self>, NoSolution> {
        tcx.type_op_normalize_fn_sig(canonicalized)
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=type_op_method | COMPLEXITY=6 | LINES=11

```rust
/// This impl is not needed, since we never normalize type outlives predicates
/// in the old solver, but is required by trait bounds to be happy.
impl<'tcx> Normalizable<'tcx> for ty::PolyTypeOutlivesPredicate<'tcx> {
    fn type_op_method(
        _tcx: TyCtxt<'tcx>,
        _canonicalized: CanonicalQueryInput<'tcx, ParamEnvAnd<'tcx, Normalize<Self>>>,
    ) -> Result<CanonicalQueryResponse<'tcx, Self>, NoSolution> {
        unreachable!("we never normalize PolyTypeOutlivesPredicate")
    }
}
```

---
*Generated by AST tracing system*
