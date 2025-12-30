// Generated macro for get_tool_config (function)
macro_rules! Depcrate_envget_tool_config {
() => {
// Module: crate::env
// Provides: {"get_tool_config"}
// Dependencies: {}
pub fn get_tool_config (workspace_dir : & Path) -> Arc < ToolConfig > { TOOL_CONFIGS . lock () . unwrap () . entry (workspace_dir . to_path_buf ()) . or_insert_with (| | { ToolConfig :: from_workspace (workspace_dir) . unwrap_or_else (| e | panic ! ("Error building config from {workspace_dir:?}: {e}")) . into () }) . clone () }
};
}
