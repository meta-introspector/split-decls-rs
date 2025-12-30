// Generated macro for Hole (struct)
macro_rules! Depcrate_collections_binary_heapHole {
() => {
// Module: crate::collections::binary_heap
// Provides: {"Hole"}
// Dependencies: {}
# [doc = " Hole represents a hole in a slice i.e., an index without valid value"] # [doc = " (because it was moved from or duplicated)."] # [doc = " In drop, `Hole` will restore the slice by filling the hole"] # [doc = " position with the value that was originally removed."] struct Hole < 'a , T : 'a > { data : & 'a mut [T] , elt : ManuallyDrop < T > , pos : usize , }
};
}
