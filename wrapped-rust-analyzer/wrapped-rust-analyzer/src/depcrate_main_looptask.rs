// Generated macro for Task (enum)
macro_rules! Depcrate_main_loopTask {
() => {
// Module: crate::main_loop
// Provides: {"Task"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum Task { Response (lsp_server :: Response) , DiscoverLinkedProjects (DiscoverProjectParam) , Retry (lsp_server :: Request) , Diagnostics (DiagnosticsTaskKind) , DiscoverTest (lsp_ext :: DiscoverTestResults) , PrimeCaches (PrimeCachesProgress) , FetchWorkspace (ProjectWorkspaceProgress) , FetchBuildData (BuildDataProgress) , LoadProcMacros (ProcMacroProgress) , BuildDepsHaveChanged , }
};
}
