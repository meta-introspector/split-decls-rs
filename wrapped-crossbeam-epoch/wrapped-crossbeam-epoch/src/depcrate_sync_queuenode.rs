// Generated macro for Node (struct)
macro_rules! Depcrate_sync_queueNode {
() => {
// Module: crate::sync::queue
// Provides: {"Node"}
// Dependencies: {}
struct Node < T > { # [doc = " The slot in which a value of type `T` can be stored."] # [doc = ""] # [doc = " The type of `data` is `MaybeUninit<T>` because a `Node<T>` doesn't always contain a `T`."] # [doc = " For example, the sentinel node in a queue never contains a value: its slot is always empty."] # [doc = " Other nodes start their life with a push operation and contain a value until it gets popped"] # [doc = " out. After that such empty nodes get added to the collector for destruction."] data : MaybeUninit < T > , next : Atomic < Node < T > > , }
};
}
