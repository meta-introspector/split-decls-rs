// Generated macro for get_box_new_payload (function)
macro_rules! Depcrate_replace_boxget_box_new_payload {
() => {
// Module: crate::replace_box
// Provides: {"get_box_new_payload"}
// Dependencies: {}
fn get_box_new_payload < 'tcx > (cx : & LateContext < '_ > , expr : & Expr < 'tcx >) -> Option < & 'tcx Expr < 'tcx > > { if let ExprKind :: Call (box_new , [arg]) = expr . kind && let ExprKind :: Path (QPath :: TypeRelative (ty , seg)) = box_new . kind && seg . ident . name == sym :: new && ty . basic_res () . is_lang_item (cx , LangItem :: OwnedBox) { Some (arg) } else { None } }
};
}
