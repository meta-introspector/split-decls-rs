// Generated macro for deref_method_same_type (function)
macro_rules! Depcrate_dereferencederef_method_same_type {
() => {
// Module: crate::dereference
// Provides: {"deref_method_same_type"}
// Dependencies: {}
fn deref_method_same_type < 'tcx > (result_ty : Ty < 'tcx > , arg_ty : Ty < 'tcx >) -> bool { match (result_ty . kind () , arg_ty . kind ()) { (ty :: Ref (_ , result_ty , _) , ty :: Ref (_ , arg_ty , _)) => result_ty == arg_ty , _ => false , } }
};
}
