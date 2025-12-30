// Generated macro for CallbackResult (enum)
macro_rules! Depcrate_eventCallbackResult {
() => {
// Module: crate::event
// Provides: {"CallbackResult"}
// Dependencies: {}
# [doc = " What the system should do with the event passed to the callback."] # [doc = ""] # [doc = " This value is ignored if [`CGEventTapOptions::ListenOnly`] is specified."] pub enum CallbackResult { # [doc = " Pass the event unchanged to other consumers."] Keep , # [doc = " Drop the event so it is not passed to later consumers."] Drop , # [doc = " Replace the event with a different one."] Replace (CGEvent) , }
};
}
