// Generated macro for impl_167 (impl)
macro_rules! Depcrate_squeueimpl_167 {
() => {
// Module: crate::squeue
// Provides: {"impl_167"}
// Dependencies: {}
impl < E : EntryMarker > Drop for SubmissionQueue < '_ , E > { # [inline] fn drop (& mut self) { unsafe { & * self . queue . tail } . store (self . tail , atomic :: Ordering :: Release) ; } }
};
}
