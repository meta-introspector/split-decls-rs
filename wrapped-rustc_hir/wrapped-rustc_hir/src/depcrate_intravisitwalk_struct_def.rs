// Generated macro for walk_struct_def (function)
macro_rules! Depcrate_intravisitwalk_struct_def {
() => {
// Module: crate::intravisit
// Provides: {"walk_struct_def"}
// Dependencies: {}
pub fn walk_struct_def < 'v , V : Visitor < 'v > > (visitor : & mut V , struct_definition : & 'v VariantData < 'v > ,) -> V :: Result { visit_opt ! (visitor , visit_id , struct_definition . ctor_hir_id ()) ; walk_list ! (visitor , visit_field_def , struct_definition . fields ()) ; V :: Result :: output () }
};
}
