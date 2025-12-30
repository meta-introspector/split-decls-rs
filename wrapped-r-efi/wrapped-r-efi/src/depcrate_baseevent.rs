// Generated macro for Event (type)
macro_rules! Depcrate_baseEvent {
() => {
// Module: crate::base
// Provides: {"Event"}
// Dependencies: {}
# [doc = " Event Objects"] # [doc = ""] # [doc = " Event objects represent hooks into the main-loop of a UEFI environment. They allow to register"] # [doc = " callbacks, to be invoked when a specific event happens. In most cases you use events to"] # [doc = " register timer-based callbacks, as well as chaining events together. Internally, they are"] # [doc = " simple void pointers. It is the UEFI task management that applies meaning to them."] pub type Event = * mut core :: ffi :: c_void ;
};
}
