// Generated macro for PerCoreScheduler (struct)
macro_rules! Depcrate_schedulerPerCoreScheduler {
() => {
// Module: crate::scheduler
// Provides: {"PerCoreScheduler"}
// Dependencies: {}
# [cfg_attr (any (target_arch = "x86_64" , target_arch = "aarch64") , repr (align (128)))] # [cfg_attr (not (any (target_arch = "x86_64" , target_arch = "aarch64")) , repr (align (64)))] pub (crate) struct PerCoreScheduler { # [doc = " Core ID of this per-core scheduler"] # [cfg (feature = "smp")] core_id : CoreId , # [doc = " Task which is currently running"] current_task : Rc < RefCell < Task > > , # [doc = " Idle Task"] idle_task : Rc < RefCell < Task > > , # [doc = " Task that currently owns the FPU"] # [cfg (any (target_arch = "x86_64" , target_arch = "aarch64"))] fpu_owner : Rc < RefCell < Task > > , # [doc = " Queue of tasks, which are ready"] ready_queue : PriorityTaskQueue , # [doc = " Queue of tasks, which are finished and can be released"] finished_tasks : VecDeque < Rc < RefCell < Task > > > , # [doc = " Queue of blocked tasks, sorted by wakeup time."] blocked_tasks : BlockedTaskQueue , }
};
}
