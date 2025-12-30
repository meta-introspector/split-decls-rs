// Generated macro for pin (function)
macro_rules! Depcrate_defaultpin {
() => {
// Module: crate::default
// Provides: {"pin"}
// Dependencies: {}
# [doc = " Pins the current thread."] # [inline] pub fn pin () -> Guard { with_handle (| handle | handle . pin ()) }
};
}
