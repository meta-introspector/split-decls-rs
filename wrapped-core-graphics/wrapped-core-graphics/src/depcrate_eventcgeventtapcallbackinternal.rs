// Generated macro for CGEventTapCallBackInternal (type)
macro_rules! Depcrate_eventCGEventTapCallBackInternal {
() => {
// Module: crate::event
// Provides: {"CGEventTapCallBackInternal"}
// Dependencies: {}
type CGEventTapCallBackInternal = unsafe extern "C" fn (proxy : CGEventTapProxy , etype : CGEventType , event : crate :: sys :: CGEventRef , user_info : * const c_void ,) -> crate :: sys :: CGEventRef ;
};
}
