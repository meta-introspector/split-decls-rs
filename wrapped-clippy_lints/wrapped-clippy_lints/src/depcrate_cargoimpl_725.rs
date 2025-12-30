// Generated macro for impl_725 (impl)
macro_rules! Depcrate_cargoimpl_725 {
() => {
// Module: crate::cargo
// Provides: {"impl_725"}
// Dependencies: {}
impl LateLintPass < '_ > for Cargo { fn check_crate (& mut self , cx : & LateContext < '_ >) { static NO_DEPS_LINTS : & [& Lint] = & [CARGO_COMMON_METADATA , REDUNDANT_FEATURE_NAMES , NEGATIVE_FEATURE_NAMES , WILDCARD_DEPENDENCIES ,] ; static WITH_DEPS_LINTS : & [& Lint] = & [MULTIPLE_CRATE_VERSIONS] ; lint_groups_priority :: check (cx) ; if ! NO_DEPS_LINTS . iter () . all (| & lint | is_lint_allowed (cx , lint , CRATE_HIR_ID)) { match MetadataCommand :: new () . no_deps () . exec () { Ok (metadata) => { common_metadata :: check (cx , & metadata , self . ignore_publish) ; feature_name :: check (cx , & metadata) ; wildcard_dependencies :: check (cx , & metadata) ; } , Err (e) => { for lint in NO_DEPS_LINTS { span_lint (cx , lint , DUMMY_SP , format ! ("could not read cargo metadata: {e}")) ; } } , } } if ! WITH_DEPS_LINTS . iter () . all (| & lint | is_lint_allowed (cx , lint , CRATE_HIR_ID)) { match MetadataCommand :: new () . exec () { Ok (metadata) => { multiple_crate_versions :: check (cx , & metadata , & self . allowed_duplicate_crates) ; } , Err (e) => { for lint in WITH_DEPS_LINTS { span_lint (cx , lint , DUMMY_SP , format ! ("could not read cargo metadata: {e}")) ; } } , } } } }
};
}
