// Generated macro for impl_456 (impl)
macro_rules! Depcrate_from_metaimpl_456 {
() => {
// Module: crate::from_meta
// Provides: {"impl_456"}
// Dependencies: {}
impl FromMeta for ident_case :: RenameRule { fn from_string (value : & str) -> Result < Self > { value . parse () . map_err (| _ | Error :: unknown_value (value)) } }
};
}
