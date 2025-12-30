// Generated macro for Inner (struct)
macro_rules! Depcrate_cqueueInner {
() => {
// Module: crate::cqueue
// Provides: {"Inner"}
// Dependencies: {}
pub (crate) struct Inner < E : EntryMarker > { head : * const atomic :: AtomicU32 , tail : * const atomic :: AtomicU32 , ring_mask : u32 , ring_entries : u32 , overflow : * const atomic :: AtomicU32 , cqes : * const E , # [allow (dead_code)] flags : * const atomic :: AtomicU32 , }
};
}
