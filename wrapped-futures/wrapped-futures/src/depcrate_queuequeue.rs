// Generated macro for Queue (struct)
macro_rules! Depcrate_queueQueue {
() => {
// Module: crate::queue
// Provides: {"Queue"}
// Dependencies: {}
pub (crate) struct Queue { state : Rc < QueueState > , promise : Promise , closure : Closure < dyn FnMut (JsValue) > , has_queue_microtask : bool , }
};
}
