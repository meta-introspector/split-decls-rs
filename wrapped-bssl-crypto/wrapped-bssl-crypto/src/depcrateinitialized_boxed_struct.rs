// Generated macro for initialized_boxed_struct (function)
macro_rules! Depcrateinitialized_boxed_struct {
() => {
// Module: crate
// Provides: {"initialized_boxed_struct"}
// Dependencies: {}
# [doc = " Returns a boxed BoringSSL structure that is initialized by some function."] # [doc = " Requires that the given function completely initializes the value."] # [doc = ""] # [doc = " Safety: the argument must fully initialize the pointed-to `T`."] # [cfg (feature = "mlalgs")] unsafe fn initialized_boxed_struct < T , F > (init : F) -> Box < T > where F : FnOnce (* mut T) , { let mut out_uninit = Box :: new (core :: mem :: MaybeUninit :: < T > :: uninit ()) ; init (out_uninit . as_mut_ptr ()) ; unsafe { out_uninit . assume_init () } }
};
}
