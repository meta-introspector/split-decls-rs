// Generated macro for initialized_boxed_struct_fallible (function)
macro_rules! Depcrateinitialized_boxed_struct_fallible {
() => {
// Module: crate
// Provides: {"initialized_boxed_struct_fallible"}
// Dependencies: {}
# [doc = " Returns a boxed BoringSSL structure that is initialized by some function."] # [doc = " Requires that the given function completely initializes the value or else"] # [doc = " returns false."] # [doc = ""] # [doc = " Safety: the argument must fully initialize the pointed-to `T` if it returns"] # [doc = " true. If it returns false then there are no safety requirements."] # [cfg (feature = "mlalgs")] unsafe fn initialized_boxed_struct_fallible < T , F > (init : F) -> Option < Box < T > > where F : FnOnce (* mut T) -> bool , { let mut out_uninit = Box :: new (core :: mem :: MaybeUninit :: < T > :: uninit ()) ; if init (out_uninit . as_mut_ptr ()) { Some (unsafe { out_uninit . assume_init () }) } else { None } }
};
}
