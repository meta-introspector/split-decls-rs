// Generated macro for TaskHandlePriorityQueue (struct)
macro_rules! Depcrate_scheduler_taskTaskHandlePriorityQueue {
() => {
// Module: crate::scheduler::task
// Provides: {"TaskHandlePriorityQueue"}
// Dependencies: {}
# [doc = " Realize a priority queue for task handles"] # [derive (Default)] pub (crate) struct TaskHandlePriorityQueue { queues : [Option < VecDeque < TaskHandle > > ; NO_PRIORITIES] , prio_bitmap : CachePadded < u64 > , }
};
}
