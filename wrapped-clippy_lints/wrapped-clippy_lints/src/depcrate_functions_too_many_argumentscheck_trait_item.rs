// Generated macro for check_trait_item (function)
macro_rules! Depcrate_functions_too_many_argumentscheck_trait_item {
() => {
// Module: crate::functions::too_many_arguments
// Provides: {"check_trait_item"}
// Dependencies: {}
pub (super) fn check_trait_item (cx : & LateContext < '_ > , item : & hir :: TraitItem < '_ > , too_many_arguments_threshold : u64) { if let hir :: TraitItemKind :: Fn (ref sig , _) = item . kind && sig . header . abi == ExternAbi :: Rust { check_arg_number (cx , sig . decl , item . span . with_hi (sig . decl . output . span () . hi ()) , too_many_arguments_threshold ,) ; } }
};
}
