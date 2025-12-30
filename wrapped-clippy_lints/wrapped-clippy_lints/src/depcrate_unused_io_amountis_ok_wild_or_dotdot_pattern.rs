// Generated macro for is_ok_wild_or_dotdot_pattern (function)
macro_rules! Depcrate_unused_io_amountis_ok_wild_or_dotdot_pattern {
() => {
// Module: crate::unused_io_amount
// Provides: {"is_ok_wild_or_dotdot_pattern"}
// Dependencies: {}
fn is_ok_wild_or_dotdot_pattern < 'a > (cx : & LateContext < 'a > , pat : & hir :: Pat < 'a >) -> bool { if let PatKind :: TupleStruct (ref path , inner_pat , _) = pat . kind && cx . qpath_res (path , pat . hir_id) . ctor_parent (cx) . is_lang_item (cx , hir :: LangItem :: ResultOk) { if matches ! (inner_pat , []) { return true ; } if let [cons_pat] = inner_pat && matches ! (cons_pat . kind , PatKind :: Wild) { return true ; } return false ; } false }
};
}
