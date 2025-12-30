// Generated macro for is_cow_into_owned (function)
macro_rules! Depcrate_methods_unnecessary_to_ownedis_cow_into_owned {
() => {
// Module: crate::methods::unnecessary_to_owned
// Provides: {"is_cow_into_owned"}
// Dependencies: {}
# [doc = " Returns true if the named method is `Cow::into_owned`."] fn is_cow_into_owned (cx : & LateContext < '_ > , method_name : Symbol , method_parent_id : DefId) -> bool { method_name == sym :: into_owned && method_parent_id . opt_impl_ty (cx) . is_diag_item (cx , sym :: Cow) }
};
}
