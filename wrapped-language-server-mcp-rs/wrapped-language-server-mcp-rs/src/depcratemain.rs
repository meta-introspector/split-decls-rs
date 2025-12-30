// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
# [tokio :: main] async fn main () -> Result < () , Box < dyn std :: error :: Error > > { tracing_subscriber :: fmt () . with_env_filter (EnvFilter :: from_default_env () . add_directive (tracing :: Level :: INFO . into ())) . with_writer (std :: io :: stderr) . with_ansi (false) . init () ; info ! ("Starting rust-analyzer MCP server") ; let workspace_root = std :: env :: current_dir () ? ; let service = RustAnalyzerMCP :: new (workspace_root) . await ? . serve (stdio ()) . await . inspect_err (| e | { error ! ("serving error: {:?}" , e) ; }) ? ; info ! ("MCP server is running") ; service . waiting () . await ? ; Ok (()) }
};
}
