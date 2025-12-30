// Generated macro for SharedVec (type)
macro_rules! Depcrate_threadSharedVec {
() => {
// Module: crate::thread
// Provides: {"SharedVec"}
// Dependencies: {}
type SharedVec < T > = Arc < Mutex < Vec < T > > > ;
};
}
