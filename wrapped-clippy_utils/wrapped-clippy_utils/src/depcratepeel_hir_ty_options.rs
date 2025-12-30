// Generated macro for peel_hir_ty_options (function)
macro_rules! Depcratepeel_hir_ty_options {
() => {
// Module: crate
// Provides: {"peel_hir_ty_options"}
// Dependencies: {}
# [doc = " Peel `Option<…>` from `hir_ty` as long as the HIR name is `Option` and it corresponds to the"] # [doc = " `core::Option<_>` type."] pub fn peel_hir_ty_options < 'tcx > (cx : & LateContext < 'tcx > , mut hir_ty : & 'tcx hir :: Ty < 'tcx >) -> & 'tcx hir :: Ty < 'tcx > { let Some (option_def_id) = cx . tcx . get_diagnostic_item (sym :: Option) else { return hir_ty ; } ; while let TyKind :: Path (QPath :: Resolved (None , path)) = hir_ty . kind && let Some (segment) = path . segments . last () && segment . ident . name == sym :: Option && let Res :: Def (DefKind :: Enum , def_id) = segment . res && def_id == option_def_id && let [GenericArg :: Type (arg_ty)] = segment . args () . args { hir_ty = arg_ty . as_unambig_ty () ; } hir_ty }
};
}
