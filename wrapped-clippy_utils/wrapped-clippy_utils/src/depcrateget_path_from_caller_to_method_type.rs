// Generated macro for get_path_from_caller_to_method_type (function)
macro_rules! Depcrateget_path_from_caller_to_method_type {
() => {
// Module: crate
// Provides: {"get_path_from_caller_to_method_type"}
// Dependencies: {}
# [doc = " Produces a path from a local caller to the type of the called method. Suitable for user"] # [doc = " output/suggestions."] # [doc = ""] # [doc = " Returned path can be either absolute (for methods defined non-locally), or relative (for local"] # [doc = " methods)."] pub fn get_path_from_caller_to_method_type < 'tcx > (tcx : TyCtxt < 'tcx > , from : LocalDefId , method : DefId , args : GenericArgsRef < 'tcx > ,) -> String { let assoc_item = tcx . associated_item (method) ; let def_id = assoc_item . container_id (tcx) ; match assoc_item . container { rustc_ty :: AssocContainer :: Trait => get_path_to_callee (tcx , from , def_id) , rustc_ty :: AssocContainer :: InherentImpl | rustc_ty :: AssocContainer :: TraitImpl (_) => { let ty = tcx . type_of (def_id) . instantiate_identity () ; get_path_to_ty (tcx , from , ty , args) } , } }
};
}
