// Generated macro for impl_272 (impl)
macro_rules! Depcrateimpl_272 {
() => {
// Module: crate
// Provides: {"impl_272"}
// Dependencies: {}
impl < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > Drop for IoUring < S , C > { fn drop (& mut self) { unsafe { ManuallyDrop :: drop (& mut self . memory) ; } } }
};
}
