// Generated macro for get_path_to_callee (function)
macro_rules! Depcrateget_path_to_callee {
() => {
// Module: crate
// Provides: {"get_path_to_callee"}
// Dependencies: {}
# [doc = " Produce a path from some local caller to the callee. Suitable for user output/suggestions."] fn get_path_to_callee (tcx : TyCtxt < '_ > , from : LocalDefId , callee : DefId) -> String { if callee . is_local () { let callee_path = tcx . def_path (callee) ; let caller_path = tcx . def_path (from . to_def_id ()) ; maybe_get_relative_path (& caller_path , & callee_path , 2) } else { tcx . def_path_str (callee) } }
};
}
