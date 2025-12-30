// Generated macro for other_2075 (other)
macro_rules! Depcrate_generatedother_2075 {
() => {
// Module: crate::generated
// Provides: {"other_2075"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Registers the given AudioObjectPropertyListenerProc to receive notifications"] # [doc = " when the given properties change."] # [doc = ""] # [doc = " Parameter `inObjectID`: The AudioObject to register the listener with."] # [doc = ""] # [doc = " Parameter `inAddress`: The AudioObjectPropertyAddresses indicating which property the listener"] # [doc = " should be notified about."] # [doc = ""] # [doc = " Parameter `inListener`: The AudioObjectPropertyListenerProc to call."] # [doc = ""] # [doc = " Parameter `inClientData`: A pointer to client data that is passed to the listener when it is called."] # [doc = ""] # [doc = " Returns: An OSStatus indicating success or failure."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_address` must be a valid pointer."] # [doc = " - `in_listener` must be implemented correctly."] # [doc = " - `in_client_data` must be a valid pointer."] # [cfg (feature = "objc2-core-audio")] # [deprecated = "no longer supported"] pub fn AudioHardwareServiceAddPropertyListener (in_object_id : AudioObjectID , in_address : * const AudioObjectPropertyAddress , in_listener : AudioObjectPropertyListenerProc , in_client_data : * mut c_void ,) -> OSStatus ; }
};
}
