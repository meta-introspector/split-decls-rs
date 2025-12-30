// Generated macro for get_manifest_contents (function)
macro_rules! Depcrate_new_lintget_manifest_contents {
() => {
// Module: crate::new_lint
// Provides: {"get_manifest_contents"}
// Dependencies: {}
fn get_manifest_contents (lint_name : & str , hint : & str) -> String { formatdoc ! (r#"
        # {hint}

        [package]
        name = "{lint_name}"
        version = "0.1.0"
        publish = false

        [workspace]
    "#) }
};
}
