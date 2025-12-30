// Generated macro for CAClockListenerProc (type)
macro_rules! Depcrate_generatedCAClockListenerProc {
() => {
// Module: crate::generated
// Provides: {"CAClockListenerProc"}
// Dependencies: {}
# [doc = " A client-supplied function called when the clock's state changes."] # [doc = ""] # [doc = ""] # [doc = " Parameter `userData`: The value passed to CAClockAddListener when the callback function"] # [doc = " was installed."] # [doc = ""] # [doc = " Parameter `message`: Signifies the kind of event which occurred."] # [doc = ""] # [doc = " Parameter `param`: This value is specific to the message (currently no messages have values)."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/caclocklistenerproc?language=objc)"] pub type CAClockListenerProc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , CAClockMessage , NonNull < c_void >) > ;
};
}
