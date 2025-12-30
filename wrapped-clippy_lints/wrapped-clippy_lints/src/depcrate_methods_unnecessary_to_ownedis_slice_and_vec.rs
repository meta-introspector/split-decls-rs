// Generated macro for is_slice_and_vec (function)
macro_rules! Depcrate_methods_unnecessary_to_ownedis_slice_and_vec {
() => {
// Module: crate::methods::unnecessary_to_owned
// Provides: {"is_slice_and_vec"}
// Dependencies: {}
fn is_slice_and_vec (cx : & LateContext < '_ > , arg_ty : Ty < '_ > , original_arg_ty : Ty < '_ >) -> bool { (original_arg_ty . is_slice () || original_arg_ty . is_array () || original_arg_ty . is_array_slice ()) && arg_ty . is_diag_item (cx , sym :: Vec) }
};
}
