// Generated macro for check (function)
macro_rules! Depcrate_cargo_wildcard_dependenciescheck {
() => {
// Module: crate::cargo::wildcard_dependencies
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , metadata : & Metadata) { for dep in & metadata . packages [0] . dependencies { if let Ok (wildcard_ver) = semver :: VersionReq :: parse ("*") && let Some (ref source) = dep . source && ! source . starts_with ("git") && dep . req == wildcard_ver { span_lint (cx , WILDCARD_DEPENDENCIES , DUMMY_SP , format ! ("wildcard dependency for `{}`" , dep . name) ,) ; } } }
};
}
