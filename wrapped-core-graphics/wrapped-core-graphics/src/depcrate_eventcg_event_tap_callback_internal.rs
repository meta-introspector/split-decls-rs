// Generated macro for cg_event_tap_callback_internal (function)
macro_rules! Depcrate_eventcg_event_tap_callback_internal {
() => {
// Module: crate::event
// Provides: {"cg_event_tap_callback_internal"}
// Dependencies: {}
unsafe extern "C" fn cg_event_tap_callback_internal (proxy : CGEventTapProxy , etype : CGEventType , event : crate :: sys :: CGEventRef , user_info : * const c_void ,) -> crate :: sys :: CGEventRef { let callback = user_info as * mut CGEventTapCallbackFn ; let event = ManuallyDrop :: new (CGEvent :: from_ptr (event)) ; let response = (* callback) (proxy , etype , & event) ; use CallbackResult :: * ; match response { Keep => event . as_ptr () , Drop => ptr :: null_mut () , Replace (new_event) => ManuallyDrop :: new (new_event) . as_ptr () , } }
};
}
