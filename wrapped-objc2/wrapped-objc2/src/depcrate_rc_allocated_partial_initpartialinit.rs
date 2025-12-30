// Generated macro for PartialInit (struct)
macro_rules! Depcrate_rc_allocated_partial_initPartialInit {
() => {
// Module: crate::rc::allocated_partial_init
// Provides: {"PartialInit"}
// Dependencies: {}
# [doc = " An Objective-C object that has been allocated and initialized in the"] # [doc = " current class, but not yet initialized in the superclass."] # [doc = ""] # [doc = " This is returned by [`Allocated::set_ivars`], and is intended to be used"] # [doc = " further in [`msg_send!`] `super` calls."] # [doc = ""] # [doc = " [`msg_send!`]: crate::msg_send"] # [doc = ""] # [doc = ""] # [doc = " # Memory layout"] # [doc = ""] # [doc = " The memory layout of this struct is NOT currently guaranteed, as we may"] # [doc = " want to be able to move a drop flag to the stack in the future."] # [repr (transparent)] # [derive (Debug)] pub struct PartialInit < T : ? Sized > { # [doc = " The partially initialized object."] # [doc = ""] # [doc = " Variance is same as `Retained`."] ptr : * const T , # [doc = " Necessary for dropck, as with `Retained`."] p : PhantomData < T > , # [doc = " Restrict auto traits, same as `Allocated<T>`."] p_auto_traits : PhantomData < AnyObject > , }
};
}
