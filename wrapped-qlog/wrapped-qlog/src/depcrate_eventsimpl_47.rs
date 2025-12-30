// Generated macro for impl_47 (impl)
macro_rules! Depcrate_eventsimpl_47 {
() => {
// Module: crate::events
// Provides: {"impl_47"}
// Dependencies: {}
impl Eventable for Event { fn importance (& self) -> EventImportance { self . ty . into () } fn set_time (& mut self , time : f32) { self . time = time ; } }
};
}
