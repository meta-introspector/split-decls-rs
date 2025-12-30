// Generated macro for impl_69 (impl)
macro_rules! Depcrate_taskimpl_69 {
() => {
// Module: crate::task
// Provides: {"impl_69"}
// Dependencies: {}
impl TaskHandle { # [doc = " Returns whether this task handle and another point to the same task."] # [doc = ""] # [doc = " In other words, this method returns whether `notify` would end up"] # [doc = " notifying the same task. If two task handles need to be notified but"] # [doc = " they are equivalent, then only one needs to be actually notified."] pub fn equivalent (& self , other : & TaskHandle) -> bool { & * self . inner as * const _ == & * other . inner as * const _ } # [doc = " Notify the associated task that a future is ready to get polled."] # [doc = ""] # [doc = " Futures should use this method to ensure that when a future can make"] # [doc = " progress as `Task` is notified that it should continue to `poll` the"] # [doc = " future at a later date."] # [doc = ""] # [doc = " Currently it's guaranteed that if `notify` is called that `poll` will be"] # [doc = " scheduled to get called at some point in the future. A `poll` may"] # [doc = " already be running on another thread, but this will ensure that a poll"] # [doc = " happens again to receive this notification."] pub fn notify (& self) { if self . inner . registered . swap (true , Ordering :: SeqCst) { return } self . inner . slot . on_full (| slot | { let (task , future) = slot . try_consume () . ok () . unwrap () ; task . handle . inner . registered . store (false , Ordering :: SeqCst) ; DEFAULT . execute (| | task . run (future)) }) ; } }
};
}
