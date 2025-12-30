// Generated macro for Slot (struct)
macro_rules! Depcrate_unboundedSlot {
() => {
// Module: crate::unbounded
// Provides: {"Slot"}
// Dependencies: {}
# [doc = " A slot in a block."] struct Slot < T > { # [doc = " The value."] value : UnsafeCell < MaybeUninit < T > > , # [doc = " The state of the slot."] state : AtomicUsize , }
};
}
