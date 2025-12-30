// Generated macro for to_html_id (function)
macro_rules! Depcrate_jsonto_html_id {
() => {
// Module: crate::json
// Provides: {"to_html_id"}
// Dependencies: {}
# [doc = " Creates a custom ID allowed by GitHub, they must start with `user-content-` and cannot contain"] # [doc = " `::`/`_`"] fn to_html_id (lint_name : & str) -> String { lint_name . replace ("clippy::" , "user-content-") . replace ('_' , "-") }
};
}
