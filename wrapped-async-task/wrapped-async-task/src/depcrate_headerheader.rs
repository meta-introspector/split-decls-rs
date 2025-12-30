// Generated macro for Header (struct)
macro_rules! Depcrate_headerHeader {
() => {
// Module: crate::header
// Provides: {"Header"}
// Dependencies: {}
pub (crate) struct Header { # [doc = " Current state of the task."] # [doc = ""] # [doc = " Contains flags representing the current state and the reference count."] pub (crate) state : AtomicUsize , # [doc = " The task that is blocked on the `Task` handle."] # [doc = ""] # [doc = " This waker needs to be woken up once the task completes or is closed."] pub (crate) awaiter : UnsafeCell < Option < Waker > > , # [doc = " The virtual table."] # [doc = ""] # [doc = " In addition to the actual waker virtual table, it also contains pointers to several other"] # [doc = " methods necessary for bookkeeping the heap-allocated task."] pub (crate) vtable : & 'static TaskVTable , # [doc = " Whether or not a panic that occurs in the task should be propagated."] # [cfg (feature = "std")] pub (crate) propagate_panic : bool , }
};
}
