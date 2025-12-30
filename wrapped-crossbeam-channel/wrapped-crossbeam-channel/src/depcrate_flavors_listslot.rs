// Generated macro for Slot (struct)
macro_rules! Depcrate_flavors_listSlot {
() => {
// Module: crate::flavors::list
// Provides: {"Slot"}
// Dependencies: {}
# [doc = " A slot in a block."] struct Slot < T > { # [doc = " The message."] msg : UnsafeCell < MaybeUninit < T > > , # [doc = " The state of the slot."] state : AtomicUsize , }
};
}
