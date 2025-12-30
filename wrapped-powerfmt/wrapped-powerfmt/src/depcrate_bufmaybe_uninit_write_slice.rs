// Generated macro for maybe_uninit_write_slice (function)
macro_rules! Depcrate_bufmaybe_uninit_write_slice {
() => {
// Module: crate::buf
// Provides: {"maybe_uninit_write_slice"}
// Dependencies: {}
# [doc = " Equivalent of [`MaybeUninit::write_slice`] that compiles on stable."] fn maybe_uninit_write_slice < 'a , T > (this : & 'a mut [MaybeUninit < T >] , src : & [T]) -> & 'a mut [T] where T : Copy , { # [allow (trivial_casts)] let uninit_src = unsafe { & * (src as * const [T] as * const [MaybeUninit < T >]) } ; this . copy_from_slice (uninit_src) ; unsafe { maybe_uninit_slice_assume_init_mut (this) } }
};
}
