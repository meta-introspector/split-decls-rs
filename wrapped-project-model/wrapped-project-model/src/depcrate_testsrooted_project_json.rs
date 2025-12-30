// Generated macro for rooted_project_json (function)
macro_rules! Depcrate_testsrooted_project_json {
() => {
// Module: crate::tests
// Provides: {"rooted_project_json"}
// Dependencies: {}
fn rooted_project_json (data : ProjectJsonData) -> ProjectJson { let mut root = "$ROOT$" . to_owned () ; replace_root (& mut root , true) ; let path = Utf8Path :: new (& root) ; let base = AbsPath :: assert (path) ; ProjectJson :: new (None , base , data) }
};
}
