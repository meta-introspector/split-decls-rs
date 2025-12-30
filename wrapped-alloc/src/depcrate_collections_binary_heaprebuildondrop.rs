// Generated macro for RebuildOnDrop (struct)
macro_rules! Depcrate_collections_binary_heapRebuildOnDrop {
() => {
// Module: crate::collections::binary_heap
// Provides: {"RebuildOnDrop"}
// Dependencies: {}
struct RebuildOnDrop < 'a , T : Ord , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator = Global , > { heap : & 'a mut BinaryHeap < T , A > , rebuild_from : usize , }
};
}
