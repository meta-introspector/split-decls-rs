// Generated macro for impl_299 (impl)
macro_rules! Depcrate_valueimpl_299 {
() => {
// Module: crate::value
// Provides: {"impl_299"}
// Dependencies: {}
impl < 'a , T : Clone + Into < Value > > From < & 'a [T] > for Value { fn from (value : & 'a [T]) -> Self { value . iter () . map (Clone :: clone) . map (Into :: into) . collect () } }
};
}
