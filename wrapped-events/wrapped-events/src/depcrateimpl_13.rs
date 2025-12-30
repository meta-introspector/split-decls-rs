// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl Drop for EventListener { # [inline] fn drop (& mut self) { if let Some (callback) = & self . callback { self . target . remove_event_listener_with_callback_and_bool (self . event_type () , callback . as_ref () . unchecked_ref () , self . phase . is_capture () ,) . unwrap_throw () ; } } }
};
}
