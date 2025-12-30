// Generated macro for join (function)
macro_rules! Depcrate_schedulerjoin {
() => {
// Module: crate::scheduler
// Provides: {"join"}
// Dependencies: {}
# [allow (clippy :: result_unit_err)] pub fn join (id : TaskId) -> Result < () , () > { let core_scheduler = core_scheduler () ; debug ! ("Task {} is waiting for task {}" , core_scheduler . get_current_task_id () , id) ; loop { let mut waiting_tasks_guard = WAITING_TASKS . lock () ; if let Some (queue) = waiting_tasks_guard . get_mut (& id) { queue . push_back (core_scheduler . get_current_task_handle ()) ; core_scheduler . block_current_task (None) ; drop (waiting_tasks_guard) ; core_scheduler . reschedule () ; } else { return Ok (()) ; } } }
};
}
