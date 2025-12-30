// Generated macro for impl_101 (impl)
macro_rules! Depcrate_ext_impls_impl_const_defaultimpl_101 {
() => {
// Module: crate::ext_impls::impl_const_default
// Provides: {"impl_101"}
// Dependencies: {}
impl < T : ConstDefault , U : ArrayLength > GenericArray < T , U > where Self : ConstDefault , { # [doc = " Returns the constant \"default value\" for an array using [ConstDefault]"] # [inline (always)] pub const fn const_default () -> Self { Self :: DEFAULT } }
};
}
