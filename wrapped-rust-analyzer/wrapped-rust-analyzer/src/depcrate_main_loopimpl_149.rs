// Generated macro for impl_149 (impl)
macro_rules! Depcrate_main_loopimpl_149 {
() => {
// Module: crate::main_loop
// Provides: {"impl_149"}
// Dependencies: {}
impl fmt :: Debug for Event { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let debug_non_verbose = | not : & Notification , f : & mut fmt :: Formatter < '_ > | { f . debug_struct ("Notification") . field ("method" , & not . method) . finish () } ; match self { Event :: Lsp (lsp_server :: Message :: Notification (not)) => { if notification_is :: < lsp_types :: notification :: DidOpenTextDocument > (not) || notification_is :: < lsp_types :: notification :: DidChangeTextDocument > (not) { return debug_non_verbose (not , f) ; } } Event :: Task (Task :: Response (resp)) => { return f . debug_struct ("Response") . field ("id" , & resp . id) . field ("error" , & resp . error) . finish () ; } _ => () , } match self { Event :: Lsp (it) => fmt :: Debug :: fmt (it , f) , Event :: Task (it) => fmt :: Debug :: fmt (it , f) , Event :: QueuedTask (it) => fmt :: Debug :: fmt (it , f) , Event :: Vfs (it) => fmt :: Debug :: fmt (it , f) , Event :: Flycheck (it) => fmt :: Debug :: fmt (it , f) , Event :: TestResult (it) => fmt :: Debug :: fmt (it , f) , Event :: DiscoverProject (it) => fmt :: Debug :: fmt (it , f) , Event :: FetchWorkspaces (it) => fmt :: Debug :: fmt (it , f) , } } }
};
}
