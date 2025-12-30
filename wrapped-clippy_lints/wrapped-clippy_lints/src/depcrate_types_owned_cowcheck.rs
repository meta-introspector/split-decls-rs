// Generated macro for check (function)
macro_rules! Depcrate_types_owned_cowcheck {
() => {
// Module: crate::types::owned_cow
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , qpath : & hir :: QPath < '_ > , def_id : DefId) -> bool { if cx . tcx . is_diagnostic_item (sym :: Cow , def_id) && let hir :: QPath :: Resolved (_ , path) = qpath && let [.. , last_seg] = path . segments && let Some (args) = last_seg . args && let [_lt , carg] = args . args && let hir :: GenericArg :: Type (cty) = carg && let Some ((span , repl)) = replacement (cx , cty . as_unambig_ty ()) { span_lint_and_sugg (cx , super :: OWNED_COW , span , "needlessly owned Cow type" , "use" , repl , Applicability :: Unspecified ,) ; return true ; } false }
};
}
