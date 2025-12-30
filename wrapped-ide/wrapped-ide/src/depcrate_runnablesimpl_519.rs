// Generated macro for impl_519 (impl)
macro_rules! Depcrate_runnablesimpl_519 {
() => {
// Module: crate::runnables
// Provides: {"impl_519"}
// Dependencies: {}
impl RunnableKind { fn disc (& self) -> RunnableDiscKind { match self { RunnableKind :: TestMod { .. } => RunnableDiscKind :: TestMod , RunnableKind :: Test { .. } => RunnableDiscKind :: Test , RunnableKind :: DocTest { .. } => RunnableDiscKind :: DocTest , RunnableKind :: Bench { .. } => RunnableDiscKind :: Bench , RunnableKind :: Bin => RunnableDiscKind :: Bin , } } }
};
}
