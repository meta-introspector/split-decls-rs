// Generated macro for impl_1164 (impl)
macro_rules! Depcrate_base_uninitimpl_1164 {
() => {
// Module: crate::base::uninit
// Provides: {"impl_1164"}
// Dependencies: {}
unsafe impl < T > InitStatus < T > for Init { type Value = T ; # [inline (always)] fn init (out : & mut T , t : T) { * out = t ; } # [inline (always)] unsafe fn assume_init_ref (t : & T) -> & T { t } # [inline (always)] unsafe fn assume_init_mut (t : & mut T) -> & mut T { t } }
};
}
