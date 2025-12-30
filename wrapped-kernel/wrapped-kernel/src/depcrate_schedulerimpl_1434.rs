// Generated macro for impl_1434 (impl)
macro_rules! Depcrate_schedulerimpl_1434 {
() => {
// Module: crate::scheduler
// Provides: {"impl_1434"}
// Dependencies: {}
impl From < NewTask > for Task { fn from (value : NewTask) -> Self { let NewTask { tid , func , arg , prio , core_id , stacks , object_map , } = value ; let mut task = Self :: new (tid , core_id , TaskStatus :: Ready , prio , stacks , object_map) ; task . create_stack_frame (func , arg) ; task } }
};
}
