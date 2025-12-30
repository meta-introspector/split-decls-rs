// Generated macro for impl_122 (impl)
macro_rules! Depcrate_boxedimpl_122 {
() => {
// Module: crate::boxed
// Provides: {"impl_122"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl < T : Default > Default for Box < T > { # [doc = " Creates a `Box<T>`, with the `Default` value for `T`."] # [inline] fn default () -> Self { let mut x : Box < mem :: MaybeUninit < T > > = Box :: new_uninit () ; unsafe { ptr :: write (& raw mut * x as * mut T , T :: default ()) ; x . assume_init () } } }
};
}
