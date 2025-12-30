// Generated macro for impl_26 (impl)
macro_rules! Depcrate_secretimpl_26 {
() => {
// Module: crate::secret
// Provides: {"impl_26"}
// Dependencies: {}
impl < T : AsRef < str > > Secret < T > { # [doc = " Checks if the contained value is empty."] pub fn is_empty (& self) -> bool { self . inner . as_ref () . is_empty () } }
};
}
