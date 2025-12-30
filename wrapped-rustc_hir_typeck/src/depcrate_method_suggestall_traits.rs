// Generated macro for all_traits (function)
macro_rules! Depcrate_method_suggestall_traits {
() => {
// Module: crate::method::suggest
// Provides: {"all_traits"}
// Dependencies: {}
# [doc = " Retrieves all traits in this crate and any dependent crates,"] # [doc = " and wraps them into `TraitInfo` for custom sorting."] pub (crate) fn all_traits (tcx : TyCtxt < '_ >) -> Vec < TraitInfo > { tcx . all_traits_including_private () . map (| def_id | TraitInfo { def_id }) . collect () }
};
}
