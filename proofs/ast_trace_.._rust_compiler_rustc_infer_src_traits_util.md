# AST Trace: ../rust/compiler/rustc_infer/src/traits/util.rs

Generated 9 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use rustc_data_structures::fx::FxHashSet;
pub use rustc_middle::ty::elaborate::*;
use rustc_middle::ty::{self, TyCtxt};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use rustc_span::{Ident, Span};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::traits::{self, Obligation, ObligationCauseCode, PredicateObligation};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=anonymize_predicate | COMPLEXITY=2 | LINES=8

```rust
pub fn anonymize_predicate<'tcx>(
    tcx: TyCtxt<'tcx>,
    pred: ty::Predicate<'tcx>,
) -> ty::Predicate<'tcx> {
    let new = tcx.anonymize_bound_vars(pred.kind());
    tcx.reuse_or_mk_predicate(pred, new)
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=STRUCT | NAME=PredicateSet | COMPLEXITY=2 | LINES=5

```rust
pub struct PredicateSet<'tcx> {
    tcx: TyCtxt<'tcx>,
    set: FxHashSet<ty::Predicate<'tcx>>,
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=new | COMPLEXITY=6 | LINES=27

```rust
impl<'tcx> PredicateSet<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self { tcx, set: Default::default() }
    }

    /// Adds a predicate to the set.
    ///
    /// Returns whether the predicate was newly inserted. That is:
    /// - If the set did not previously contain this predicate, `true` is returned.
    /// - If the set already contained this predicate, `false` is returned,
    ///   and the set is not modified: original predicate is not replaced,
    ///   and the predicate passed as argument is dropped.
    pub fn insert(&mut self, pred: ty::Predicate<'tcx>) -> bool {
        // We have to be careful here because we want
        //
        //    for<'a> Foo<&'a i32>
        //
        // and
        //
        //    for<'b> Foo<&'b i32>
        //
        // to be considered equivalent. So normalize all late-bound
        // regions before we throw things into the underlying set.
        self.set.insert(anonymize_predicate(self.tcx, pred))
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=extend | COMPLEXITY=10 | LINES=16

```rust
impl<'tcx> Extend<ty::Predicate<'tcx>> for PredicateSet<'tcx> {
    fn extend<I: IntoIterator<Item = ty::Predicate<'tcx>>>(&mut self, iter: I) {
        for pred in iter {
            self.insert(pred);
        }
    }

    fn extend_one(&mut self, pred: ty::Predicate<'tcx>) {
        self.insert(pred);
    }

    fn extend_reserve(&mut self, additional: usize) {
        Extend::<ty::Predicate<'tcx>>::extend_reserve(&mut self.set, additional);
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=predicate | COMPLEXITY=13 | LINES=40

```rust
/// For [`Obligation`], a sub-obligation is combined with the current obligation's
/// param-env and cause code.
impl<'tcx> Elaboratable<TyCtxt<'tcx>> for PredicateObligation<'tcx> {
    fn predicate(&self) -> ty::Predicate<'tcx> {
        self.predicate
    }

    fn child(&self, clause: ty::Clause<'tcx>) -> Self {
        Obligation {
            cause: self.cause.clone(),
            param_env: self.param_env,
            recursion_depth: 0,
            predicate: clause.as_predicate(),
        }
    }

    fn child_with_derived_cause(
        &self,
        clause: ty::Clause<'tcx>,
        span: Span,
        parent_trait_pred: ty::PolyTraitPredicate<'tcx>,
        index: usize,
    ) -> Self {
        let cause = self.cause.clone().derived_cause(parent_trait_pred, |derived| {
            ObligationCauseCode::ImplDerived(Box::new(traits::ImplDerivedCause {
                derived,
                impl_or_alias_def_id: parent_trait_pred.def_id(),
                impl_def_predicate_index: Some(index),
                span,
            }))
        });
        Obligation {
            cause,
            param_env: self.param_env,
            recursion_depth: 0,
            predicate: clause.as_predicate(),
        }
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=transitive_bounds_that_define_assoc_item | COMPLEXITY=11 | LINES=35

```rust
/// A specialized variant of `elaborate` that only elaborates trait references that may
/// define the given associated item with the name `assoc_name`. It uses the
/// `explicit_supertraits_containing_assoc_item` query to avoid enumerating super-predicates that
/// aren't related to `assoc_item`. This is used when resolving types like `Self::Item` or
/// `T::Item` and helps to avoid cycle errors (see e.g. #35237).
pub fn transitive_bounds_that_define_assoc_item<'tcx>(
    tcx: TyCtxt<'tcx>,
    trait_refs: impl Iterator<Item = ty::PolyTraitRef<'tcx>>,
    assoc_name: Ident,
) -> impl Iterator<Item = ty::PolyTraitRef<'tcx>> {
    let mut seen = FxHashSet::default();
    let mut stack: Vec<_> = trait_refs.collect();

    std::iter::from_fn(move || {
        while let Some(trait_ref) = stack.pop() {
            if !seen.insert(tcx.anonymize_bound_vars(trait_ref)) {
                continue;
            }

            stack.extend(
                tcx.explicit_supertraits_containing_assoc_item((trait_ref.def_id(), assoc_name))
                    .iter_identity_copied()
                    .map(|(clause, _)| clause.instantiate_supertrait(tcx, trait_ref))
                    .filter_map(|clause| clause.as_trait_clause())
                    .filter(|clause| clause.polarity() == ty::PredicatePolarity::Positive)
                    .map(|clause| clause.map_bound(|clause| clause.trait_ref)),
            );

            return Some(trait_ref);
        }

        None
    })
}
```

---
*Generated by AST tracing system*
