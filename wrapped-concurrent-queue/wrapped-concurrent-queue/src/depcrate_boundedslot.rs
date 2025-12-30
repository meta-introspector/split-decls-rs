// Generated macro for Slot (struct)
macro_rules! Depcrate_boundedSlot {
() => {
// Module: crate::bounded
// Provides: {"Slot"}
// Dependencies: {}
# [doc = " A slot in a queue."] struct Slot < T > { # [doc = " The current stamp."] stamp : AtomicUsize , # [doc = " The value in this slot."] value : UnsafeCell < MaybeUninit < T > > , }
};
}
