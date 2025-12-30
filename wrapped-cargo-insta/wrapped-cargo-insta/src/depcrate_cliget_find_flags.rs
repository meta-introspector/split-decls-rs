// Generated macro for get_find_flags (function)
macro_rules! Depcrate_cliget_find_flags {
() => {
// Module: crate::cli
// Provides: {"get_find_flags"}
// Dependencies: {}
fn get_find_flags (tool_config : & ToolConfig , target_args : & TargetArgs) -> FindFlags { FindFlags { include_ignored : target_args . include_ignored || tool_config . review_include_ignored () , include_hidden : target_args . include_hidden || tool_config . review_include_hidden () , } }
};
}
