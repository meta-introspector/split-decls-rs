// Generated macro for get_some (function)
macro_rules! Depcrate_matches_manual_unwrap_orget_some {
() => {
// Module: crate::matches::manual_unwrap_or
// Provides: {"get_some"}
// Dependencies: {}
fn get_some (cx : & LateContext < '_ > , pat : & Pat < '_ >) -> Option < HirId > { if let PatKind :: TupleStruct (QPath :: Resolved (_ , path) , & [pat] , _) = pat . kind && let PatKind :: Binding (BindingMode (ByRef :: No , _) , pat_id , _ , _) = pat . kind && let Some (def_id) = path . res . opt_def_id () && let Some (def_id) = cx . tcx . opt_parent (def_id) && let Some (lang_item) = cx . tcx . lang_items () . from_def_id (def_id) && matches ! (lang_item , LangItem :: OptionSome | LangItem :: ResultOk) { Some (pat_id) } else { None } }
};
}
