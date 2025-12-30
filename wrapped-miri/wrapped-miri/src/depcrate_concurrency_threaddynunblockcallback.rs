// Generated macro for DynUnblockCallback (type)
macro_rules! Depcrate_concurrency_threadDynUnblockCallback {
() => {
// Module: crate::concurrency::thread
// Provides: {"DynUnblockCallback"}
// Dependencies: {}
# [doc = " Type alias for unblock callbacks, i.e. machine callbacks invoked when"] # [doc = " a thread gets unblocked."] pub type DynUnblockCallback < 'tcx > = DynMachineCallback < 'tcx , UnblockKind > ;
};
}
