// Generated macro for is_plain_default (function)
macro_rules! Depcrate_box_defaultis_plain_default {
() => {
// Module: crate::box_default
// Provides: {"is_plain_default"}
// Dependencies: {}
fn is_plain_default (cx : & LateContext < '_ > , arg_path : & Expr < '_ >) -> bool { if let ExprKind :: Path (QPath :: Resolved (None , path)) = & arg_path . kind && let Res :: Def (_ , def_id) = path . res { cx . tcx . is_diagnostic_item (sym :: default_fn , def_id) && path . segments . iter () . all (| seg | seg . args . is_none ()) } else { false } }
};
}
