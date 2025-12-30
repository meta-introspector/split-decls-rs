// Generated macro for Slot (struct)
macro_rules! Depcrate_dequeSlot {
() => {
// Module: crate::deque
// Provides: {"Slot"}
// Dependencies: {}
# [doc = " A slot in a block."] struct Slot < T > { # [doc = " The task."] task : UnsafeCell < MaybeUninit < T > > , # [doc = " The state of the slot."] state : AtomicUsize , }
};
}
