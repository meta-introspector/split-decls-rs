// Generated macro for maybe_uninit_uninit_array (function)
macro_rules! Depcrate_bufmaybe_uninit_uninit_array {
() => {
// Module: crate::buf
// Provides: {"maybe_uninit_uninit_array"}
// Dependencies: {}
# [doc = " Equivalent of [`MaybeUninit::uninit_array`] that compiles on stable."] # [must_use] # [inline (always)] const fn maybe_uninit_uninit_array < T , const N : usize > () -> [MaybeUninit < T > ; N] { unsafe { MaybeUninit :: < [MaybeUninit < T > ; N] > :: uninit () . assume_init () } }
};
}
