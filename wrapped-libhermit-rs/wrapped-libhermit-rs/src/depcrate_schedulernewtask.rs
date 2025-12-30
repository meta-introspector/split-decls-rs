// Generated macro for NewTask (struct)
macro_rules! Depcrate_schedulerNewTask {
() => {
// Module: crate::scheduler
// Provides: {"NewTask"}
// Dependencies: {}
struct NewTask { tid : TaskId , func : unsafe extern "C" fn (usize) , arg : usize , prio : Priority , core_id : CoreId , stacks : TaskStacks , object_map : Arc < RwSpinLock < HashMap < FileDescriptor , Arc < async_lock :: RwLock < dyn ObjectInterface > > , RandomState > , > , > , }
};
}
