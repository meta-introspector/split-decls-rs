// Generated macro for WorkStealingQueue (struct)
macro_rules! Depcrate_work_queueWorkStealingQueue {
() => {
// Module: crate::work_queue
// Provides: {"WorkStealingQueue"}
// Dependencies: {}
# [doc = " A work-stealing queue that supports multiple workers taking work from a shared queue."] # [doc = ""] # [doc = " Will be removed once core::sync::mpsc is stable."] pub (crate) struct WorkStealingQueue < T > { inner : Arc < Inner < T > > , }
};
}
