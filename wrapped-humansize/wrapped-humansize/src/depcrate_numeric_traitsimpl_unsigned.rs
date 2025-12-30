// Generated macro for impl_unsigned (macro)
macro_rules! Depcrate_numeric_traitsimpl_unsigned {
() => {
// Module: crate::numeric_traits
// Provides: {"impl_unsigned"}
// Dependencies: {}
macro_rules ! impl_unsigned { (for $ ($ t : ty) *) => ($ (impl Signed for $ t { }) *) }
};
}
