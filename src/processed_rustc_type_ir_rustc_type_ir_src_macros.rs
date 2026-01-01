// SRC: ../rust/compiler/rustc_type_ir/src/macros.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=21 | LINES=36 */
/// Used for types that are `Copy` and which **do not care arena
/// allocated data** (i.e., don't need to be folded).
#[macro_export]
macro_rules! TrivialTypeTraversalImpls {
    ($($ty:ty,)+) => {
        $(
            impl<I: $crate::Interner> $crate::TypeFoldable<I> for $ty {
                fn try_fold_with<F: $crate::FallibleTypeFolder<I>>(
                    self,
                    _: &mut F,
                ) -> ::std::result::Result<Self, F::Error> {
                    Ok(self)
                }

                #[inline]
                fn fold_with<F: $crate::TypeFolder<I>>(
                    self,
                    _: &mut F,
                ) -> Self {
                    self
                }
            }

            impl<I: $crate::Interner> $crate::TypeVisitable<I> for $ty {
                #[inline]
                fn visit_with<F: $crate::TypeVisitor<I>>(
                    &self,
                    _: &mut F)
                    -> F::Result
                {
                    <F::Result as $crate::VisitorResult>::output()
                }
            }
        )+
    };
}
/* AST_META: AST_ID=2 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=27 */

///////////////////////////////////////////////////////////////////////////
// Atomic structs
//
// For things that don't carry any arena-allocated data (and are
// copy...), just add them to this list.

TrivialTypeTraversalImpls! {
    (),
    bool,
    usize,
    u16,
    u32,
    u64,
    // tidy-alphabetical-start
    crate::AliasRelationDirection,
    crate::BoundConstness,
    crate::DebruijnIndex,
    crate::PredicatePolarity,
    crate::UniverseIndex,
    crate::Variance,
    crate::solve::BuiltinImplSource,
    crate::solve::Certainty,
    crate::solve::GoalSource,
    rustc_ast_ir::Mutability,
    // tidy-alphabetical-end
}