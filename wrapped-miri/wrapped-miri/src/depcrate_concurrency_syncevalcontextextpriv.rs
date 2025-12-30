// Generated macro for EvalContextExtPriv (trait)
macro_rules! Depcrate_concurrency_syncEvalContextExtPriv {
() => {
// Module: crate::concurrency::sync
// Provides: {"EvalContextExtPriv"}
// Dependencies: {}
pub (super) trait EvalContextExtPriv < 'tcx > : crate :: MiriInterpCxExt < 'tcx > { fn condvar_reacquire_mutex (& mut self , mutex_ref : MutexRef , retval : Scalar , dest : MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx > { let this = self . eval_context_mut () ; if let Some (owner) = mutex_ref . owner () { assert_ne ! (owner , this . active_thread ()) ; this . mutex_enqueue_and_block (mutex_ref , Some ((retval , dest))) ; } else { this . mutex_lock (& mutex_ref) ; this . write_scalar (retval , & dest) ? ; } interp_ok (()) } }
};
}
