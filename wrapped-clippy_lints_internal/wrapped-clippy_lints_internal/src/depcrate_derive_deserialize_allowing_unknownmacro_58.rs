// Generated macro for macro_58 (macro)
macro_rules! Depcrate_derive_deserialize_allowing_unknownmacro_58 {
() => {
// Module: crate::derive_deserialize_allowing_unknown
// Provides: {"macro_58"}
// Dependencies: {}
declare_tool_lint ! { # [doc = " ### What it does"] # [doc = " Checks for structs or enums that derive `serde::Deserialize` and that"] # [doc = " do not have a `#[serde(deny_unknown_fields)]` attribute."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " If the struct or enum is used in [`clippy_config::conf::Conf`] and a"] # [doc = " user inserts an unknown field by mistake, the user's error will be"] # [doc = " silently ignored."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust"] # [doc = " #[derive(serde::Deserialize)]"] # [doc = " pub struct DisallowedPath {"] # [doc = "     path: String,"] # [doc = "     reason: Option<String>,"] # [doc = "     replacement: Option<String>,"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust"] # [doc = " #[derive(serde::Deserialize)]"] # [doc = " #[serde(deny_unknown_fields)]"] # [doc = " pub struct DisallowedPath {"] # [doc = "     path: String,"] # [doc = "     reason: Option<String>,"] # [doc = "     replacement: Option<String>,"] # [doc = " }"] # [doc = " ```"] pub clippy :: DERIVE_DESERIALIZE_ALLOWING_UNKNOWN , Allow , "`#[derive(serde::Deserialize)]` without `#[serde(deny_unknown_fields)]`" , report_in_external_macro : true }
};
}
