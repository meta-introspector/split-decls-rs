// Generated macro for normalize_type_name (function)
macro_rules! Depcrate_abi_examplenormalize_type_name {
() => {
// Module: crate::abi_example
// Provides: {"normalize_type_name"}
// Dependencies: {}
pub (crate) fn normalize_type_name (type_name : & str) -> String { type_name . chars () . filter (| c | * c != '&') . collect () }
};
}
