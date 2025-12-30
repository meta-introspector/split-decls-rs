// Generated macro for walk_field_def (function)
macro_rules! Depcrate_intravisitwalk_field_def {
() => {
// Module: crate::intravisit
// Provides: {"walk_field_def"}
// Dependencies: {}
pub fn walk_field_def < 'v , V : Visitor < 'v > > (visitor : & mut V , FieldDef { hir_id , ident , ty , default , span : _ , vis_span : _ , def_id : _ , safety : _ } : & 'v FieldDef < 'v > ,) -> V :: Result { try_visit ! (visitor . visit_id (* hir_id)) ; try_visit ! (visitor . visit_ident (* ident)) ; visit_opt ! (visitor , visit_anon_const , default) ; visitor . visit_ty_unambig (* ty) }
};
}
