// Generated macro for all_names (function)
macro_rules! Depcrate_extern_abiall_names {
() => {
// Module: crate::extern_abi
// Provides: {"all_names"}
// Dependencies: {}
pub fn all_names () -> Vec < & 'static str > { ExternAbi :: ALL_VARIANTS . iter () . map (| abi | abi . as_str ()) . collect () }
};
}
