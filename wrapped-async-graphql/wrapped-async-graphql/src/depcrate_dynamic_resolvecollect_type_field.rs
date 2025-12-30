// Generated macro for collect_type_field (function)
macro_rules! Depcrate_dynamic_resolvecollect_type_field {
() => {
// Module: crate::dynamic::resolve
// Provides: {"collect_type_field"}
// Dependencies: {}
fn collect_type_field < 'a > (fields : & mut Vec < BoxFieldFuture < 'a > > , ctx : & ContextSelectionSet < 'a > , field : & 'a Positioned < Field > ,) { let ctx = ctx . clone () ; fields . push (async move { let ctx_field = ctx . with_field (field) ; let (_ , type_name) = ctx_field . param_value :: < String > ("name" , None) ? ; let mut ctx_obj = ctx . with_selection_set (& ctx_field . item . node . selection_set) ; ctx_obj . is_for_introspection = true ; let visible_types = ctx . schema_env . registry . find_visible_types (& ctx_field) ; let value = crate :: OutputType :: resolve (& ctx . schema_env . registry . types . get (& type_name) . filter (| _ | visible_types . contains (type_name . as_str ())) . map (| ty | { crate :: model :: __Type :: new_simple (& ctx . schema_env . registry , & visible_types , ty ,) }) , & ctx_obj , ctx_field . item ,) . await ? ; Ok ((field . node . response_key () . node . clone () , value)) } . boxed () ,) ; }
};
}
