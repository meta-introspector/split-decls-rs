// Generated macro for impl_1165 (impl)
macro_rules! Depcrate_base_uninitimpl_1165 {
() => {
// Module: crate::base::uninit
// Provides: {"impl_1165"}
// Dependencies: {}
unsafe impl < T > InitStatus < T > for Uninit { type Value = MaybeUninit < T > ; # [inline (always)] fn init (out : & mut MaybeUninit < T > , t : T) { * out = MaybeUninit :: new (t) ; } # [inline (always)] unsafe fn assume_init_ref (t : & MaybeUninit < T >) -> & T { & * t . as_ptr () } # [inline (always)] unsafe fn assume_init_mut (t : & mut MaybeUninit < T >) -> & mut T { & mut * t . as_mut_ptr () } }
};
}
