// Generated macro for impl_45 (impl)
macro_rules! Depcrateimpl_45 {
() => {
// Module: crate
// Provides: {"impl_45"}
// Dependencies: {}
# [tool_handler] impl ServerHandler for RustAnalyzerMCP { fn get_info (& self) -> ServerInfo { ServerInfo { protocol_version : ProtocolVersion :: V_2024_11_05 , capabilities : ServerCapabilities :: builder () . enable_tools () . build () , server_info : Implementation :: from_build_env () , instructions : Some ("This server provides rust-analyzer functionality through MCP tools. Available tools: 'hover' for type information, 'completion' for code completions, 'diagnostics' for compile errors, 'goto_definition' to find definitions, 'find_references' to find all references, 'format_document' to format code, 'rename' to rename symbols across the workspace, 'code_actions' to get quick fixes and refactorings, 'workspace_symbols' to search symbols across the workspace, 'inlay_hints' to get type and parameter hints, 'expand_macro' to expand Rust macros, 'document_symbols' for code structure analysis, 'signature_help' for function parameter assistance, 'document_highlight' for symbol occurrence highlighting, 'selection_range' for smart selection expansion, 'runnables' to find tests, benchmarks, and executables, and 'implementations' to find all implementations of a trait." . to_string ()) , } } async fn initialize (& self , _request : InitializeRequestParam , _context : RequestContext < RoleServer > ,) -> Result < InitializeResult , McpError > { Ok (self . get_info ()) } }
};
}
