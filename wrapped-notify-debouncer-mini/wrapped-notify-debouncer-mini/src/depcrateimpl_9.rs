// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl < F > DebounceEventHandler for F where F : FnMut (DebounceEventResult) + Send + 'static , { fn handle_event (& mut self , event : DebounceEventResult) { (self) (event) ; } }
};
}
