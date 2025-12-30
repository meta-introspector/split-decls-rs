// Generated macro for eq_closure_binder (function)
macro_rules! Depcrate_ast_utilseq_closure_binder {
() => {
// Module: crate::ast_utils
// Provides: {"eq_closure_binder"}
// Dependencies: {}
pub fn eq_closure_binder (l : & ClosureBinder , r : & ClosureBinder) -> bool { match (l , r) { (ClosureBinder :: NotPresent , ClosureBinder :: NotPresent) => true , (ClosureBinder :: For { generic_params : lp , .. } , ClosureBinder :: For { generic_params : rp , .. }) => { lp . len () == rp . len () && std :: iter :: zip (lp . iter () , rp . iter ()) . all (| (l , r) | eq_generic_param (l , r)) } , _ => false , } }
};
}
