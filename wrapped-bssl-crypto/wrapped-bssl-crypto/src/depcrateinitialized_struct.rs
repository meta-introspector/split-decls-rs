// Generated macro for initialized_struct (function)
macro_rules! Depcrateinitialized_struct {
() => {
// Module: crate
// Provides: {"initialized_struct"}
// Dependencies: {}
# [doc = " Returns a BoringSSL structure that is initialized by some function."] # [doc = " Requires that the given function completely initializes the value."] # [doc = ""] # [doc = " (Tagged `unsafe` because a no-op argument would otherwise expose"] # [doc = " uninitialized memory.)"] unsafe fn initialized_struct < T , F > (init : F) -> T where F : FnOnce (* mut T) , { let mut out_uninit = core :: mem :: MaybeUninit :: < T > :: uninit () ; init (out_uninit . as_mut_ptr ()) ; unsafe { out_uninit . assume_init () } }
};
}
