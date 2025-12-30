// Generated macro for collect_entities_field (function)
macro_rules! Depcrate_dynamic_resolvecollect_entities_field {
() => {
// Module: crate::dynamic::resolve
// Provides: {"collect_entities_field"}
// Dependencies: {}
fn collect_entities_field < 'a > (fields : & mut Vec < BoxFieldFuture < 'a > > , schema : & 'a Schema , ctx : & ContextSelectionSet < 'a > , parent_value : & 'a FieldValue , field : & 'a Positioned < Field > ,) { let ctx = ctx . clone () ; fields . push (async move { let ctx_field = ctx . with_field (field) ; let entity_resolver = schema . 0 . entity_resolver . as_ref () . ok_or_else (| | { ctx_field . set_error_path (Error :: new ("internal: missing entity resolver") . into_server_error (ctx_field . item . pos) ,) }) ? ; let entity_type = TypeRef :: named_list_nn ("_Entity") ; let arguments = ObjectAccessor (Cow :: Owned (field . node . arguments . iter () . map (| (name , value) | { ctx_field . resolve_input_value (value . clone ()) . map (| value | (name . node . clone () , value)) }) . collect :: < ServerResult < IndexMap < Name , Value > > > () ? ,)) ; let field_future = (entity_resolver) (ResolverContext { ctx : & ctx_field , args : arguments , parent_value , }) ; let field_value = match field_future { FieldFuture :: Future (fut) => { fut . await . map_err (| err | err . into_server_error (field . pos)) ? } FieldFuture :: Value (value) => value , } ; let value = resolve (schema , & ctx_field , & entity_type , field_value . as_ref ()) . await ? . unwrap_or_default () ; Ok ((field . node . response_key () . node . clone () , value)) } . boxed () ,) ; }
};
}
