// Generated macro for Node (struct)
macro_rules! Depcrate_baseNode {
() => {
// Module: crate::base
// Provides: {"Node"}
// Dependencies: {}
# [doc = " A skip list node."] # [doc = ""] # [doc = " This struct is marked with `repr(C)` so that the specific order of fields is enforced."] # [doc = " It is important that the tower is the last field since it is dynamically sized. The key,"] # [doc = " reference count, and height are kept close to the tower to improve cache locality during"] # [doc = " skip list traversal."] # [repr (C)] struct Node < K , V > { # [doc = " The value."] value : V , # [doc = " The key."] key : K , # [doc = " Keeps the reference count and the height of its tower."] # [doc = ""] # [doc = " The reference count is equal to the number of `Entry`s pointing to this node, plus the"] # [doc = " number of levels in which this node is installed."] refs_and_height : AtomicUsize , # [doc = " The tower of atomic pointers."] tower : Tower < K , V > , }
};
}
