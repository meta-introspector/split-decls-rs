// Generated macro for Slot (struct)
macro_rules! Depcrate_array_queueSlot {
() => {
// Module: crate::array_queue
// Provides: {"Slot"}
// Dependencies: {}
# [doc = " A slot in a queue."] struct Slot < T > { # [doc = " The current stamp."] # [doc = ""] # [doc = " If the stamp equals the tail, this node will be next written to. If it equals head + 1,"] # [doc = " this node will be next read from."] stamp : AtomicUsize , # [doc = " The value in this slot."] value : UnsafeCell < MaybeUninit < T > > , }
};
}
