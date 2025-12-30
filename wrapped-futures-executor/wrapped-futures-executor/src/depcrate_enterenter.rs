// Generated macro for enter (function)
macro_rules! Depcrate_enterenter {
() => {
// Module: crate::enter
// Provides: {"enter"}
// Dependencies: {}
# [doc = " Marks the current thread as being within the dynamic extent of an"] # [doc = " executor."] # [doc = ""] # [doc = " Executor implementations should call this function before beginning to"] # [doc = " execute a task, and drop the returned [`Enter`](Enter) value after"] # [doc = " completing task execution:"] # [doc = ""] # [doc = " ```"] # [doc = " use futures::executor::enter;"] # [doc = ""] # [doc = " let enter = enter().expect(\"...\");"] # [doc = " /* run task */"] # [doc = " drop(enter);"] # [doc = " ```"] # [doc = ""] # [doc = " Doing so ensures that executors aren't"] # [doc = " accidentally invoked in a nested fashion."] # [doc = ""] # [doc = " # Error"] # [doc = ""] # [doc = " Returns an error if the current thread is already marked, in which case the"] # [doc = " caller should panic with a tailored error message."] pub fn enter () -> Result < Enter , EnterError > { ENTERED . with (| c | { if c . get () { Err (EnterError { _priv : () }) } else { c . set (true) ; Ok (Enter { _priv : () }) } }) }
};
}
