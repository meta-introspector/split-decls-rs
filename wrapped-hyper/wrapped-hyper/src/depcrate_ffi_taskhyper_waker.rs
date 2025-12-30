// Generated macro for hyper_waker (struct)
macro_rules! Depcrate_ffi_taskhyper_waker {
() => {
// Module: crate::ffi::task
// Provides: {"hyper_waker"}
// Dependencies: {}
# [doc = " A waker that is saved and used to waken a pending task."] # [doc = ""] # [doc = " This is provided to `hyper_io`'s read and write callbacks via `hyper_context`"] # [doc = " and `hyper_context_waker`."] # [doc = ""] # [doc = " When nonblocking I/O in one of those callbacks can't make progress (returns"] # [doc = " `EAGAIN` or `EWOULDBLOCK`), the callback has to return to avoid blocking the"] # [doc = " executor. But it also has to arrange to get called in the future when more"] # [doc = " data is available. That's the role of the async context and the waker. The"] # [doc = " waker can be used to tell the executor \"this task is ready to make progress.\""] # [doc = ""] # [doc = " The read or write callback, upon finding it can't make progress, must get a"] # [doc = " waker from the context (`hyper_context_waker`), arrange for that waker to be"] # [doc = " called in the future, and then return `HYPER_POLL_PENDING`."] # [doc = ""] # [doc = " The arrangements for the waker to be called in the future are up to the"] # [doc = " application, but usually it will involve one big `select(2)` loop that checks which"] # [doc = " FDs are ready, and a correspondence between FDs and waker objects. For each"] # [doc = " FD that is ready, the corresponding waker must be called. Then `hyper_executor_poll`"] # [doc = " must be called. That will cause the executor to attempt to make progress on each"] # [doc = " woken task."] # [doc = ""] # [doc = " Corresponding Rust type: <https://doc.rust-lang.org/std/task/struct.Waker.html>"] pub struct hyper_waker { waker : std :: task :: Waker , }
};
}
