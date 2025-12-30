// Generated macro for IntrusiveArrayBuilder (struct)
macro_rules! Depcrate_internalIntrusiveArrayBuilder {
() => {
// Module: crate::internal
// Provides: {"IntrusiveArrayBuilder"}
// Dependencies: {}
# [doc = " Similar to [`ArrayBuilder`] but uses a reference to a pre-allocated array, be"] # [doc = " it on the stack or heap."] pub struct IntrusiveArrayBuilder < 'a , T , N : ArrayLength > { array : & 'a mut GenericArray < MaybeUninit < T > , N > , position : usize , }
};
}
