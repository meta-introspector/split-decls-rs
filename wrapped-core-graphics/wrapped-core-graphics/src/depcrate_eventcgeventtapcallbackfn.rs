// Generated macro for CGEventTapCallbackFn (type)
macro_rules! Depcrate_eventCGEventTapCallbackFn {
() => {
// Module: crate::event
// Provides: {"CGEventTapCallbackFn"}
// Dependencies: {}
type CGEventTapCallbackFn < 'tap_life > = Box < dyn Fn (CGEventTapProxy , CGEventType , & CGEvent) -> CallbackResult + 'tap_life > ;
};
}
