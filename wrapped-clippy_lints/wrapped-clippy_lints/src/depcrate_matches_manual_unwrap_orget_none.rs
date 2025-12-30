// Generated macro for get_none (function)
macro_rules! Depcrate_matches_manual_unwrap_orget_none {
() => {
// Module: crate::matches::manual_unwrap_or
// Provides: {"get_none"}
// Dependencies: {}
fn get_none < 'tcx > (cx : & LateContext < '_ > , arm : & Arm < 'tcx > , allow_wildcard : bool) -> Option < & 'tcx Expr < 'tcx > > { if let PatKind :: Expr (PatExpr { kind : PatExprKind :: Path (QPath :: Resolved (_ , path)) , .. }) = arm . pat . kind && let Some (def_id) = path . res . opt_def_id () && let Some (def_id) = cx . tcx . opt_parent (def_id) && cx . tcx . lang_items () . get (LangItem :: OptionNone) == Some (def_id) { Some (arm . body) } else if let PatKind :: TupleStruct (QPath :: Resolved (_ , path) , _ , _) = arm . pat . kind && let Some (def_id) = path . res . opt_def_id () && let Some (def_id) = cx . tcx . opt_parent (def_id) && cx . tcx . lang_items () . get (LangItem :: ResultErr) == Some (def_id) { Some (arm . body) } else if let PatKind :: Wild = arm . pat . kind && allow_wildcard { Some (arm . body) } else { None } }
};
}
