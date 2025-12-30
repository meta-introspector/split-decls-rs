// Generated macro for other_2076 (other)
macro_rules! Depcrate_generatedother_2076 {
() => {
// Module: crate::generated
// Provides: {"other_2076"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Unregisters the given AudioObjectPropertyListenerProc from receiving"] # [doc = " notifications when the given properties change."] # [doc = ""] # [doc = " Parameter `inObjectID`: The AudioObject to unregister the listener from."] # [doc = ""] # [doc = " Parameter `inAddress`: The AudioObjectPropertyAddresses indicating which property the listener"] # [doc = " will stop being notified about."] # [doc = ""] # [doc = " Parameter `inListener`: The AudioObjectPropertyListenerProc being removed."] # [doc = ""] # [doc = " Parameter `inClientData`: A pointer to client data that is passed to the listener when it is called."] # [doc = ""] # [doc = " Returns: An OSStatus indicating success or failure."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_address` must be a valid pointer."] # [doc = " - `in_listener` must be implemented correctly."] # [doc = " - `in_client_data` must be a valid pointer."] # [cfg (feature = "objc2-core-audio")] # [deprecated = "no longer supported"] pub fn AudioHardwareServiceRemovePropertyListener (in_object_id : AudioObjectID , in_address : * const AudioObjectPropertyAddress , in_listener : AudioObjectPropertyListenerProc , in_client_data : * mut c_void ,) -> OSStatus ; }
};
}
