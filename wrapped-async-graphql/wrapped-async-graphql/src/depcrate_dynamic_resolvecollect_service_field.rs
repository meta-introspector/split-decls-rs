// Generated macro for collect_service_field (function)
macro_rules! Depcrate_dynamic_resolvecollect_service_field {
() => {
// Module: crate::dynamic::resolve
// Provides: {"collect_service_field"}
// Dependencies: {}
fn collect_service_field < 'a > (fields : & mut Vec < BoxFieldFuture < 'a > > , ctx : & ContextSelectionSet < 'a > , field : & 'a Positioned < Field > ,) { let ctx = ctx . clone () ; fields . push (async move { let ctx_field = ctx . with_field (field) ; let mut ctx_obj = ctx . with_selection_set (& ctx_field . item . node . selection_set) ; ctx_obj . is_for_introspection = true ; let output_type = crate :: OutputType :: resolve (& Service { sdl : Some (ctx . schema_env . registry . export_sdl (SDLExportOptions :: new () . federation () . compose_directive ()) ,) , } , & ctx_obj , ctx_field . item ,) . await ? ; Ok ((field . node . response_key () . node . clone () , output_type)) } . boxed () ,) ; }
};
}
