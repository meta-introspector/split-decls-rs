// Generated macro for impl_583 (impl)
macro_rules! Depcrate_drivers_virtio_virtqueue_packedimpl_583 {
() => {
// Module: crate::drivers::virtio::virtqueue::packed
// Provides: {"impl_583"}
// Dependencies: {}
impl RingIndexRange for ops :: Range < RingIdx > { fn wrapping_contains (& self , item : & RingIdx) -> bool { let ops :: Range { start , end } = self ; if start . wrap == end . wrap { item . wrap == start . wrap && start . off <= item . off && item . off < end . off } else if item . wrap == start . wrap { start . off <= item . off } else { debug_assert ! (item . wrap == end . wrap) ; item . off < end . off } } }
};
}
