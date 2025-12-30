// Generated macro for Ctx (struct)
macro_rules! Depcrate_path_transformCtx {
() => {
// Module: crate::path_transform
// Provides: {"Ctx"}
// Dependencies: {}
struct Ctx < 'a > { type_substs : FxHashMap < hir :: TypeParam , ast :: Type > , const_substs : FxHashMap < hir :: ConstParam , SyntaxNode > , lifetime_substs : FxHashMap < LifetimeName , ast :: Lifetime > , target_module : hir :: Module , source_scope : & 'a SemanticsScope < 'a > , same_self_type : bool , target_edition : Edition , }
};
}
