// Generated macro for resolve_list (function)
macro_rules! Depcrate_dynamic_resolveresolve_list {
() => {
// Module: crate::dynamic::resolve
// Provides: {"resolve_list"}
// Dependencies: {}
async fn resolve_list < 'a > (schema : & 'a Schema , ctx : & 'a Context < 'a > , type_ref : & 'a TypeRef , values : & [FieldValue < '_ >] ,) -> ServerResult < Option < Value > > { let mut futures = Vec :: with_capacity (values . len ()) ; for (idx , value) in values . iter () . enumerate () { let ctx_item = ctx . with_index (idx) ; futures . push (async move { let parent_type = format ! ("[{}]" , type_ref) ; let return_type = type_ref . to_string () ; let resolve_info = ResolveInfo { path_node : ctx_item . path_node . as_ref () . unwrap () , parent_type : & parent_type , return_type : & return_type , name : ctx . item . node . name . node . as_str () , alias : ctx . item . node . alias . as_ref () . map (| alias | alias . node . as_str ()) , is_for_introspection : ctx_item . is_for_introspection , field : & ctx_item . item . node , } ; let resolve_fut = async { resolve (schema , & ctx_item , type_ref , Some (value)) . await } ; futures_util :: pin_mut ! (resolve_fut) ; let res_value = ctx_item . query_env . extensions . resolve (resolve_info , & mut resolve_fut) . await ? ; Ok :: < _ , ServerError > (res_value . unwrap_or_default ()) }) ; } let values = futures_util :: future :: try_join_all (futures) . await ? ; Ok (Some (Value :: List (values))) }
};
}
