// Generated macro for impl_323 (impl)
macro_rules! Depcrate_valueimpl_323 {
() => {
// Module: crate::value
// Provides: {"impl_323"}
// Dependencies: {}
impl < T > From < Vec < T > > for ValueKind where T : Into < Value > , { fn from (values : Vec < T >) -> Self { Self :: Array (values . into_iter () . map (T :: into) . collect ()) } }
};
}
