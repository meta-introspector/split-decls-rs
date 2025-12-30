// Generated macro for StackEmptyCallback (type)
macro_rules! Depcrate_concurrency_threadStackEmptyCallback {
() => {
// Module: crate::concurrency::thread
// Provides: {"StackEmptyCallback"}
// Dependencies: {}
pub type StackEmptyCallback < 'tcx > = Box < dyn FnMut (& mut MiriInterpCx < 'tcx >) -> InterpResult < 'tcx , Poll < () > > + 'tcx > ;
};
}
