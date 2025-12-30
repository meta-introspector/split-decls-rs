// Generated macro for impl_41 (impl)
macro_rules! Depcrate_implsimpl_41 {
() => {
// Module: crate::impls
// Provides: {"impl_41"}
// Dependencies: {}
impl < T : Default , N : ArrayLength > Default for GenericArray < T , N > { # [inline (always)] fn default () -> Self { Self :: generate (| _ | T :: default ()) } }
};
}
