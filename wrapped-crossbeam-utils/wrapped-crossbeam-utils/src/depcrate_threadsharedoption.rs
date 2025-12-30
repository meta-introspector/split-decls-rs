// Generated macro for SharedOption (type)
macro_rules! Depcrate_threadSharedOption {
() => {
// Module: crate::thread
// Provides: {"SharedOption"}
// Dependencies: {}
type SharedOption < T > = Arc < Mutex < Option < T > > > ;
};
}
