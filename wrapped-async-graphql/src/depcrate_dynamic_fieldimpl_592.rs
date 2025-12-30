// Generated macro for impl_592 (impl)
macro_rules! Depcrate_dynamic_fieldimpl_592 {
() => {
// Module: crate::dynamic::field
// Provides: {"impl_592"}
// Dependencies: {}
impl < 'a , T : Into < FieldValue < 'a > > > From < Vec < T > > for FieldValue < 'a > { fn from (values : Vec < T >) -> Self { Self (FieldValueInner :: List (values . into_iter () . map (Into :: into) . collect () ,)) } }
};
}
