// Generated macro for GLOBAL_INSTRUMENTATION (static)
macro_rules! Depcrate_connection_instrumentationGLOBAL_INSTRUMENTATION {
() => {
// Module: crate::connection::instrumentation
// Provides: {"GLOBAL_INSTRUMENTATION"}
// Dependencies: {}
static GLOBAL_INSTRUMENTATION : std :: sync :: RwLock < fn () -> Option < Box < dyn Instrumentation > > > = std :: sync :: RwLock :: new (| | None) ;
};
}
