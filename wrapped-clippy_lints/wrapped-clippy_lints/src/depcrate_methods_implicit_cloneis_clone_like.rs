// Generated macro for is_clone_like (function)
macro_rules! Depcrate_methods_implicit_cloneis_clone_like {
() => {
// Module: crate::methods::implicit_clone
// Provides: {"is_clone_like"}
// Dependencies: {}
# [doc = " Returns true if the named method can be used to clone the receiver."] pub fn is_clone_like (cx : & LateContext < '_ > , method_name : Symbol , method_parent_id : hir :: def_id :: DefId) -> bool { match method_name { sym :: to_os_string => method_parent_id . opt_impl_ty (cx) . is_diag_item (cx , sym :: OsStr) , sym :: to_owned => method_parent_id . is_diag_item (cx , sym :: ToOwned) , sym :: to_path_buf => method_parent_id . opt_impl_ty (cx) . is_diag_item (cx , sym :: Path) , sym :: to_string => method_parent_id . is_diag_item (cx , sym :: ToString) , sym :: to_vec => method_parent_id . opt_impl_ty (cx) . is_some_and (| ty | ty . instantiate_identity () . is_slice ()) , _ => false , } }
};
}
