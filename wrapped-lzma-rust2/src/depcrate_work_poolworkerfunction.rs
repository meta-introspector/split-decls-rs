// Generated macro for WorkerFunction (type)
macro_rules! Depcrate_work_poolWorkerFunction {
() => {
// Module: crate::work_pool
// Provides: {"WorkerFunction"}
// Dependencies: {}
pub (crate) type WorkerFunction < W , R > = fn (WorkerHandle < (u64 , W) > , SyncSender < (u64 , R) > , Arc < AtomicBool > , Arc < Mutex < Option < io :: Error > > > , Arc < AtomicU32 > ,) ;
};
}
