// Generated macro for is_from_ignored_trait (function)
macro_rules! Depcrate_functions_renamed_function_paramsis_from_ignored_trait {
() => {
// Module: crate::functions::renamed_function_params
// Provides: {"is_from_ignored_trait"}
// Dependencies: {}
fn is_from_ignored_trait (of_trait : & TraitRef < '_ > , ignored_traits : & DefIdSet) -> bool { of_trait . trait_def_id () . is_some_and (| trait_did | ignored_traits . contains (& trait_did)) }
};
}
