// Generated macro for extract_enum_variant (function)
macro_rules! Depcrate_derivable_implsextract_enum_variant {
() => {
// Module: crate::derivable_impls
// Provides: {"extract_enum_variant"}
// Dependencies: {}
fn extract_enum_variant < 'tcx > (cx : & LateContext < 'tcx > , func_expr : & 'tcx Expr < 'tcx > , adt_def : AdtDef < 'tcx > ,) -> Option < & 'tcx VariantDef > { match & peel_blocks (func_expr) . kind { ExprKind :: Path (QPath :: Resolved (None , p)) if let Res :: Def (DefKind :: Ctor (CtorOf :: Variant , CtorKind :: Const) , id) = p . res && let variant_id = cx . tcx . parent (id) && let Some (variant_def) = adt_def . variants () . iter () . find (| v | v . def_id == variant_id) => { Some (variant_def) } , ExprKind :: Path (QPath :: TypeRelative (ty , segment)) if let TyKind :: Path (QPath :: Resolved (None , p)) = & ty . kind && let Res :: SelfTyAlias { is_trait_impl : true , .. } = p . res && let variant_ident = segment . ident && let Some (variant_def) = adt_def . variants () . iter () . find (| v | v . ident (cx . tcx) == variant_ident) => { Some (variant_def) } , _ => None , } }
};
}
