// Generated macro for impl_1480 (impl)
macro_rules! Depcrate_synch_recmuteximpl_1480 {
() => {
// Module: crate::synch::recmutex
// Provides: {"impl_1480"}
// Dependencies: {}
impl RecursiveMutex { pub const fn new () -> Self { Self { state : TicketMutex :: new (RecursiveMutexState { current_tid : None , count : 0 , queue : TaskHandlePriorityQueue :: new () , }) , } } pub fn acquire (& self) { let core_scheduler = core_scheduler () ; let tid = core_scheduler . get_current_task_id () ; loop { { let mut locked_state = self . state . lock () ; if let Some (current_tid) = locked_state . current_tid { if current_tid == tid { locked_state . count += 1 ; return ; } } else { locked_state . current_tid = Some (tid) ; locked_state . count = 1 ; return ; } core_scheduler . block_current_task (None) ; locked_state . queue . push (core_scheduler . get_current_task_handle ()) ; } core_scheduler . reschedule () ; } } pub fn release (& self) { if let Some (task) = { let mut locked_state = self . state . lock () ; if locked_state . count > 0 { locked_state . count -= 1 ; } if locked_state . count == 0 { locked_state . current_tid = None ; locked_state . queue . pop () } else { None } } { core_scheduler () . custom_wakeup (task) ; } } }
};
}
