// Generated macro for non_consuming_err_arm (function)
macro_rules! Depcrate_unused_io_amountnon_consuming_err_arm {
() => {
// Module: crate::unused_io_amount
// Provides: {"non_consuming_err_arm"}
// Dependencies: {}
fn non_consuming_err_arm < 'a > (cx : & LateContext < 'a > , arm : & hir :: Arm < 'a >) -> bool { if arm . guard . is_some () { return false ; } if is_unreachable_or_panic (cx , arm . body) { return false ; } if let PatKind :: TupleStruct (ref path , [inner_pat] , _) = arm . pat . kind { return cx . qpath_res (path , inner_pat . hir_id) . ctor_parent (cx) . is_lang_item (cx , hir :: LangItem :: ResultErr) ; } false }
};
}
