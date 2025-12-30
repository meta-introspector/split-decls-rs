// Generated macro for Slot (struct)
macro_rules! Depcrate_flavors_arraySlot {
() => {
// Module: crate::flavors::array
// Provides: {"Slot"}
// Dependencies: {}
# [doc = " A slot in a channel."] struct Slot < T > { # [doc = " The current stamp."] stamp : AtomicUsize , # [doc = " The message in this slot."] msg : UnsafeCell < MaybeUninit < T > > , }
};
}
