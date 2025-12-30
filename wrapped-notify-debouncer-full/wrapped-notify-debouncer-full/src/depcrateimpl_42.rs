// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
impl < F > DebounceEventHandler for F where F : FnMut (DebounceEventResult) + Send + 'static , { fn handle_event (& mut self , event : DebounceEventResult) { (self) (event) ; } }
};
}
