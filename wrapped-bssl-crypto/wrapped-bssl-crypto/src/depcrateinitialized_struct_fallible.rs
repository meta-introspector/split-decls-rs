// Generated macro for initialized_struct_fallible (function)
macro_rules! Depcrateinitialized_struct_fallible {
() => {
// Module: crate
// Provides: {"initialized_struct_fallible"}
// Dependencies: {}
# [doc = " Returns a BoringSSL structure that is initialized by some function."] # [doc = " Requires that the given function completely initializes the value or else"] # [doc = " returns false."] # [doc = ""] # [doc = " (Tagged `unsafe` because a no-op argument would otherwise expose"] # [doc = " uninitialized memory.)"] unsafe fn initialized_struct_fallible < T , F > (init : F) -> Option < T > where F : FnOnce (* mut T) -> bool , { let mut out_uninit = core :: mem :: MaybeUninit :: < T > :: uninit () ; if init (out_uninit . as_mut_ptr ()) { Some (unsafe { out_uninit . assume_init () }) } else { None } }
};
}
