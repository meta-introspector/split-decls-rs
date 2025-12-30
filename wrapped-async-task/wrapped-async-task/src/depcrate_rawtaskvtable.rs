// Generated macro for TaskVTable (struct)
macro_rules! Depcrate_rawTaskVTable {
() => {
// Module: crate::raw
// Provides: {"TaskVTable"}
// Dependencies: {}
# [doc = " The vtable for a task."] pub (crate) struct TaskVTable { pub (crate) raw_waker_vtable : & 'static RawWakerVTable , # [doc = " Schedules the task."] pub (crate) schedule : unsafe fn (* const () , ScheduleInfo) , # [doc = " Drops the future inside the task."] pub (crate) drop_future : unsafe fn (* const () , & TaskLayout) , # [doc = " Destroys the task."] pub (crate) destroy : unsafe fn (* const ()) , # [doc = " Runs the task."] pub (crate) run : unsafe fn (* const ()) -> bool , # [doc = " The memory layout of the task. This information enables"] # [doc = " debuggers to decode raw task memory blobs. Do not remove"] # [doc = " the field, even if it appears to be unused."] pub (crate) layout_info : & 'static TaskLayout , }
};
}
