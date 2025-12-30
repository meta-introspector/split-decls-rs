// Generated macro for QueueState (struct)
macro_rules! Depcrate_queueQueueState {
() => {
// Module: crate::queue
// Provides: {"QueueState"}
// Dependencies: {}
struct QueueState { tasks : RefCell < VecDeque < Rc < crate :: task :: Task > > > , is_scheduled : Cell < bool > , }
};
}
