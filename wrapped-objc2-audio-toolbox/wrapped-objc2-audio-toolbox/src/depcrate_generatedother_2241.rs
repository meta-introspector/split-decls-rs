// Generated macro for other_2241 (other)
macro_rules! Depcrate_generatedother_2241 {
() => {
// Module: crate::generated
// Provides: {"other_2241"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Adds a callback function to receive notifications of changes to the clock's"] # [doc = " state."] # [doc = ""] # [doc = " Note: The CAClockListenerProc may be called on a realtime thread internal to"] # [doc = " the clock object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inCAClock`: The clock object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inListenerProc`: The callback function."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inUserData`: This value is passed to the callback function, in the userData"] # [doc = " parameter."] # [doc = ""] # [doc = ""] # [doc = " Returns: An OSStatus error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_ca_clock` must be a valid pointer."] # [doc = " - `in_listener_proc` must be implemented correctly."] # [doc = " - `in_user_data` must be a valid pointer."] pub fn CAClockAddListener (in_ca_clock : CAClockRef , in_listener_proc : CAClockListenerProc , in_user_data : NonNull < c_void > ,) -> OSStatus ; }
};
}
