// Generated macro for get_tid (function)
macro_rules! Depcrate_schedulerget_tid {
() => {
// Module: crate::scheduler
// Provides: {"get_tid"}
// Dependencies: {}
fn get_tid () -> TaskId { static TID_COUNTER : AtomicI32 = AtomicI32 :: new (0) ; let guard = TASKS . lock () ; loop { let id = TaskId :: from (TID_COUNTER . fetch_add (1 , Ordering :: SeqCst)) ; if ! guard . contains_key (& id) { return id ; } } }
};
}
