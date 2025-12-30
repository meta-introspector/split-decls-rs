// Generated macro for PriorityTaskQueue (struct)
macro_rules! Depcrate_scheduler_taskPriorityTaskQueue {
() => {
// Module: crate::scheduler::task
// Provides: {"PriorityTaskQueue"}
// Dependencies: {}
# [doc = " Realize a priority queue for tasks"] pub (crate) struct PriorityTaskQueue { queues : [LinkedList < Rc < RefCell < Task > > > ; NO_PRIORITIES] , prio_bitmap : u64 , }
};
}
