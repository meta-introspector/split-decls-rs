// Generated macro for Task (struct)
macro_rules! Depcrate_scheduler_taskTask {
() => {
// Module: crate::scheduler::task
// Provides: {"Task"}
// Dependencies: {}
# [doc = " A task control block, which identifies either a process or a thread"] # [cfg_attr (any (target_arch = "x86_64" , target_arch = "aarch64") , repr (align (128)))] # [cfg_attr (not (any (target_arch = "x86_64" , target_arch = "aarch64")) , repr (align (64)))] pub (crate) struct Task { # [doc = " The ID of this context"] pub id : TaskId , # [doc = " Status of a task, e.g. if the task is ready or blocked"] pub status : TaskStatus , # [doc = " Task priority,"] pub prio : Priority , # [doc = " Last stack pointer before a context switch to another task"] pub last_stack_pointer : VirtAddr , # [doc = " Last stack pointer on the user stack before jumping to kernel space"] pub user_stack_pointer : VirtAddr , # [doc = " Last FPU state before a context switch to another task using the FPU"] pub last_fpu_state : arch :: processor :: FPUState , # [doc = " ID of the core this task is running on"] pub core_id : CoreId , # [doc = " Stack of the task"] pub stacks : TaskStacks , # [doc = " Mapping between file descriptor and the referenced IO interface"] pub object_map : Arc < RwSpinLock < HashMap < FileDescriptor , Arc < async_lock :: RwLock < dyn ObjectInterface > > , RandomState > , > , > , # [doc = " Task Thread-Local-Storage (TLS)"] # [cfg (not (feature = "common-os"))] pub tls : Option < Tls > , # [cfg (all (target_arch = "x86_64" , feature = "common-os"))] pub root_page_table : usize , }
};
}
