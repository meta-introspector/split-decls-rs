// Generated macro for impl_541 (impl)
macro_rules! Depcrate_runnablesimpl_541 {
() => {
// Module: crate::runnables
// Provides: {"impl_541"}
// Dependencies: {}
impl RunnableKind { fn disc (& self) -> RunnableDiscKind { match self { RunnableKind :: TestMod { .. } => RunnableDiscKind :: TestMod , RunnableKind :: Test { .. } => RunnableDiscKind :: Test , RunnableKind :: DocTest { .. } => RunnableDiscKind :: DocTest , RunnableKind :: Bench { .. } => RunnableDiscKind :: Bench , RunnableKind :: Bin => RunnableDiscKind :: Bin , } } }
};
}
