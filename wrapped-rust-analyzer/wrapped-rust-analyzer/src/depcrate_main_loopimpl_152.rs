// Generated macro for impl_152 (impl)
macro_rules! Depcrate_main_loopimpl_152 {
() => {
// Module: crate::main_loop
// Provides: {"impl_152"}
// Dependencies: {}
impl fmt :: Display for Event { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Event :: Lsp (_) => write ! (f , "Event::Lsp") , Event :: Task (_) => write ! (f , "Event::Task") , Event :: Vfs (_) => write ! (f , "Event::Vfs") , Event :: Flycheck (_) => write ! (f , "Event::Flycheck") , Event :: QueuedTask (_) => write ! (f , "Event::QueuedTask") , Event :: TestResult (_) => write ! (f , "Event::TestResult") , Event :: DiscoverProject (_) => write ! (f , "Event::DiscoverProject") , Event :: FetchWorkspaces (_) => write ! (f , "Event::SwitchWorkspaces") , } } }
};
}
