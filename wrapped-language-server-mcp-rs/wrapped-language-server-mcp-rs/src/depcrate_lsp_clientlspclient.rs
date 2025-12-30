// Generated macro for LspClient (struct)
macro_rules! Depcrate_lsp_clientLspClient {
() => {
// Module: crate::lsp_client
// Provides: {"LspClient"}
// Dependencies: {}
pub struct LspClient { process : Child , stdin : Mutex < tokio :: process :: ChildStdin > , stdout : Mutex < BufReader < tokio :: process :: ChildStdout > > , request_id : Mutex < i64 > , workspace_root : PathBuf , is_ready : Arc < AtomicBool > , opened_documents : Mutex < HashSet < String > > , }
};
}
