// Generated macro for impl_795 (impl)
macro_rules! Depcrate_base_conversionimpl_795 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_795"}
// Dependencies: {}
impl < 'a , T : Scalar > From < DVectorView < 'a , T > > for & 'a [T] { fn from (vec : DVectorView < 'a , T >) -> & 'a [T] { vec . data . into_slice () } }
};
}
