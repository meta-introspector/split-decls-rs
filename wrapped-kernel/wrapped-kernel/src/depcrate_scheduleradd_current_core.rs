// Generated macro for add_current_core (function)
macro_rules! Depcrate_scheduleradd_current_core {
() => {
// Module: crate::scheduler
// Provides: {"add_current_core"}
// Dependencies: {}
# [doc = " Add a per-core scheduler for the current core."] pub (crate) fn add_current_core () { let core_id = core_id () ; let tid = get_tid () ; let idle_task = Rc :: new (RefCell :: new (Task :: new_idle (tid , core_id))) ; WAITING_TASKS . lock () . insert (tid , VecDeque :: with_capacity (1)) ; TASKS . lock () . insert (tid , TaskHandle :: new (tid , IDLE_PRIO , # [cfg (feature = "smp")] core_id ,) ,) ; debug ! ("Initializing scheduler for core {core_id} with idle task {tid}") ; let boxed_scheduler = Box :: new (PerCoreScheduler { # [cfg (feature = "smp")] core_id , current_task : idle_task . clone () , # [cfg (any (target_arch = "x86_64" , target_arch = "aarch64"))] fpu_owner : idle_task . clone () , idle_task , ready_queue : PriorityTaskQueue :: new () , finished_tasks : VecDeque :: new () , blocked_tasks : BlockedTaskQueue :: new () , }) ; let scheduler = Box :: into_raw (boxed_scheduler) ; set_core_scheduler (scheduler) ; # [cfg (feature = "smp")] { SCHEDULER_INPUTS . lock () . insert (core_id . try_into () . unwrap () , & CoreLocal :: get () . scheduler_input ,) ; # [cfg (target_arch = "x86_64")] CORE_HLT_STATE . lock () . insert (core_id . try_into () . unwrap () , & CoreLocal :: get () . hlt) ; } }
};
}
