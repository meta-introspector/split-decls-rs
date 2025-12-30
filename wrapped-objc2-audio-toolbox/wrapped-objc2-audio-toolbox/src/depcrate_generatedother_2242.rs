// Generated macro for other_2242 (other)
macro_rules! Depcrate_generatedother_2242 {
() => {
// Module: crate::generated
// Provides: {"other_2242"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Removes a listener callback function."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inCAClock`: The clock object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inListenerProc`: The callback function."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inUserData`: The same value as was passed for inUserData when this"] # [doc = " function was registered with CAClockAddListener. (This"] # [doc = " allows a single callback function to be registered more"] # [doc = " than once, with different userData arguments.)"] # [doc = ""] # [doc = ""] # [doc = " Returns: An OSStatus error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_ca_clock` must be a valid pointer."] # [doc = " - `in_listener_proc` must be implemented correctly."] # [doc = " - `in_user_data` must be a valid pointer."] pub fn CAClockRemoveListener (in_ca_clock : CAClockRef , in_listener_proc : CAClockListenerProc , in_user_data : NonNull < c_void > ,) -> OSStatus ; }
};
}
