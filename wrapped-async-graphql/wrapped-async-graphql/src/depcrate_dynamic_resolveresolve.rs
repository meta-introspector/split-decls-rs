// Generated macro for resolve (function)
macro_rules! Depcrate_dynamic_resolveresolve {
() => {
// Module: crate::dynamic::resolve
// Provides: {"resolve"}
// Dependencies: {}
pub (crate) fn resolve < 'a > (schema : & 'a Schema , ctx : & 'a Context < 'a > , type_ref : & 'a TypeRef , value : Option < & 'a FieldValue > ,) -> BoxFuture < 'a , ServerResult < Option < Value > > > { async move { match (type_ref , value) { (TypeRef :: Named (type_name) , Some (value)) => { resolve_value (schema , ctx , & schema . 0 . types [type_name . as_ref ()] , value) . await } (TypeRef :: Named (_) , None) => Ok (None) , (TypeRef :: NonNull (type_ref) , Some (value)) => { resolve (schema , ctx , type_ref , Some (value)) . await } (TypeRef :: NonNull (_) , None) => Err (ctx . set_error_path (Error :: new ("internal: non-null types require a return value") . into_server_error (ctx . item . pos) ,)) , (TypeRef :: List (type_ref) , Some (FieldValue (FieldValueInner :: List (values)))) => { resolve_list (schema , ctx , type_ref , values) . await } (TypeRef :: List (type_ref) , Some (FieldValue (FieldValueInner :: Value (Value :: List (values)))) ,) => { let values = values . iter () . cloned () . map (FieldValue :: value) . collect :: < Vec < _ > > () ; resolve_list (schema , ctx , type_ref , & values) . await } (TypeRef :: List (_) , Some (_)) => Err (ctx . set_error_path (Error :: new ("internal: expects an array") . into_server_error (ctx . item . pos) ,)) , (TypeRef :: List (_) , None) => Ok (None) , } } . boxed () }
};
}
