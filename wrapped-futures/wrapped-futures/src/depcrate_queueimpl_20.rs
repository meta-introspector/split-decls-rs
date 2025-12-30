// Generated macro for impl_20 (impl)
macro_rules! Depcrate_queueimpl_20 {
() => {
// Module: crate::queue
// Provides: {"impl_20"}
// Dependencies: {}
impl Queue { pub (crate) fn schedule_task (& self , task : Rc < crate :: task :: Task >) { self . state . tasks . borrow_mut () . push_back (task) ; if ! self . state . is_scheduled . replace (true) { if self . has_queue_microtask { queueMicrotask (& self . closure) ; } else { let _ = self . promise . then (& self . closure) ; } } } # [cfg (not (target_feature = "atomics"))] pub (crate) fn push_task (& self , task : Rc < crate :: task :: Task >) { self . schedule_task (task) } }
};
}
