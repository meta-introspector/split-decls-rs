// Generated macro for impl_891 (impl)
macro_rules! Depcrate_hir_ty_loweringimpl_891 {
() => {
// Module: crate::hir_ty_lowering
// Provides: {"impl_891"}
// Dependencies: {}
impl AssocItemQSelf { fn to_string (& self , tcx : TyCtxt < '_ >) -> String { match * self { Self :: Trait (def_id) => tcx . def_path_str (def_id) , Self :: TyParam (def_id , _) => tcx . hir_ty_param_name (def_id) . to_string () , Self :: SelfTyAlias => kw :: SelfUpper . to_string () , } } }
};
}
