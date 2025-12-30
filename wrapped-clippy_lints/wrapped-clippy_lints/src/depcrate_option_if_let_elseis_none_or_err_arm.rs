// Generated macro for is_none_or_err_arm (function)
macro_rules! Depcrate_option_if_let_elseis_none_or_err_arm {
() => {
// Module: crate::option_if_let_else
// Provides: {"is_none_or_err_arm"}
// Dependencies: {}
fn is_none_or_err_arm (cx : & LateContext < '_ > , arm : & Arm < '_ >) -> bool { match arm . pat . kind { _ if is_none_pattern (cx , arm . pat) => true , PatKind :: TupleStruct (ref qpath , [first_pat] , _) => { cx . qpath_res (qpath , arm . pat . hir_id) . ctor_parent (cx) . is_lang_item (cx , ResultErr) && matches ! (first_pat . kind , PatKind :: Wild) } , PatKind :: Wild => true , _ => false , } }
};
}
