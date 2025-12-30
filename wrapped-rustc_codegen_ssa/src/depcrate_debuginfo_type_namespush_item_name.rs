// Generated macro for push_item_name (function)
macro_rules! Depcrate_debuginfo_type_namespush_item_name {
() => {
// Module: crate::debuginfo::type_names
// Provides: {"push_item_name"}
// Dependencies: {}
pub fn push_item_name (tcx : TyCtxt < '_ > , def_id : DefId , qualified : bool , output : & mut String) { let def_key = tcx . def_key (def_id) ; if qualified && let Some (parent) = def_key . parent { push_item_name (tcx , DefId { krate : def_id . krate , index : parent } , true , output) ; output . push_str ("::") ; } push_unqualified_item_name (tcx , def_id , def_key . disambiguated_data , output) ; }
};
}
