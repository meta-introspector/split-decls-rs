// Generated macro for impl_32 (impl)
macro_rules! Depcrate_cqueueimpl_32 {
() => {
// Module: crate::cqueue
// Provides: {"impl_32"}
// Dependencies: {}
impl < E : EntryMarker > Drop for CompletionQueue < '_ , E > { # [inline] fn drop (& mut self) { unsafe { & * self . queue . head } . store (self . head , atomic :: Ordering :: Release) ; } }
};
}
