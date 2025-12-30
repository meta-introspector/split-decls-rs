// Generated macro for is_to_owned_like (function)
macro_rules! Depcrate_methods_unnecessary_to_ownedis_to_owned_like {
() => {
// Module: crate::methods::unnecessary_to_owned
// Provides: {"is_to_owned_like"}
// Dependencies: {}
# [doc = " Returns true if the named method can be used to convert the receiver to its \"owned\""] # [doc = " representation."] fn is_to_owned_like < 'a > (cx : & LateContext < 'a > , call_expr : & Expr < 'a > , method_name : Symbol , method_parent_id : DefId ,) -> bool { is_cow_into_owned (cx , method_name , method_parent_id) || (method_name != sym :: to_string && is_clone_like (cx , method_name , method_parent_id)) || is_to_string_on_string_like (cx , call_expr , method_name , method_parent_id) }
};
}
