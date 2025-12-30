// Generated macro for Event (enum)
macro_rules! Depcrate_main_loopEvent {
() => {
// Module: crate::main_loop
// Provides: {"Event"}
// Dependencies: {}
enum Event { Lsp (lsp_server :: Message) , Task (Task) , QueuedTask (QueuedTask) , Vfs (vfs :: loader :: Message) , Flycheck (FlycheckMessage) , TestResult (CargoTestMessage) , DiscoverProject (DiscoverProjectMessage) , FetchWorkspaces (FetchWorkspaceRequest) , }
};
}
