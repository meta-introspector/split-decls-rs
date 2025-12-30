// Generated macro for impl_1406 (impl)
macro_rules! Depcrate_scheduler_taskimpl_1406 {
() => {
// Module: crate::scheduler::task
// Provides: {"impl_1406"}
// Dependencies: {}
impl TaskHandle { pub fn new (id : TaskId , priority : Priority , # [cfg (feature = "smp")] core_id : CoreId) -> Self { Self { id , priority , # [cfg (feature = "smp")] core_id , } } # [cfg (feature = "smp")] pub fn get_core_id (& self) -> CoreId { self . core_id } pub fn get_id (& self) -> TaskId { self . id } pub fn get_priority (& self) -> Priority { self . priority } }
};
}
